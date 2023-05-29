#![no_std]
#![no_main]

use core::fmt::Debug;
use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Styled};

use cortex_m::interrupt::{free as disable_interrupts, CriticalSection};
use embedded_graphics::primitives::StyledDrawable;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{interrupt, CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{button_interrupt, Button, ButtonController, ButtonEvent};

// Queues used for button stuff (just normal wio stuff) and then i use it for
// managing clear queue
use heapless::{
    consts::{U64, U8},
    spsc::Queue,
};

// pseudo-random number generation
use oorandom;
use oorandom::Rand32;

const DISPLAY_WIDTH: u32 = 320;
const DISPLAY_HEIGHT: u32 = 240;
const CELL_SIZE: u32 = 10;
const GRID_WIDTH: u32 = DISPLAY_WIDTH / CELL_SIZE as u32;
const GRID_HEIGHT: u32 = DISPLAY_HEIGHT / CELL_SIZE as u32;

static mut BUTTON_CTRLR: Option<ButtonController> = None;
static mut Q: Queue<ButtonEvent, U8> = Queue(heapless::i::Queue::new());

button_interrupt!(
    BUTTON_CTRLR,
    unsafe fn on_button_event(_cs: &CriticalSection, event: ButtonEvent) {
        let mut q = Q.split().0;
        q.enqueue(event).ok();
    }
);

#[entry]
fn main() -> ! {
    // Initial initializations
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let sets = wio::Pins::new(peripherals.PORT).split();
    let mut uled = sets.user_led.into_push_pull_output();
    uled.set_low().unwrap();
    let mut consumer = unsafe { Q.split().1 };

    // initializing styles
    let black_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();

    let food_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::CSS_ORANGE)
        .build();

    let snake_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::WHITE)
        .build();

    // Initialize the ILI9341-based LCD display. Create a black backdrop the size of
    // the screen, load an image of Ferris from a RAW file, and draw it to the
    // screen.
    // By default, the display is in the LandscapeFlipped orientation.
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            100.MHz(),
            &mut delay,
        )
        .unwrap();

    // Initializing backdrop and initial sprite render
    Rectangle::new(Point::new(0, 0), Size::new(360, 240))
        .into_styled(black_style)
        .draw(&mut display)
        .unwrap();

    let mut player = Snake::init();
    player.translate(&mut display);

    // initializing buttons
    let button_ctrlr = sets
        .buttons
        .init(peripherals.EIC, &mut clocks, &mut peripherals.MCLK);
    let nvic = &mut core.NVIC;
    disable_interrupts(|_| unsafe {
        button_ctrlr.enable(nvic);
        BUTTON_CTRLR = Some(button_ctrlr);
    });

    let mut food = Food::init_and_draw(5, &food_style, &snake_style, &mut display);

    let mut flag_incr_snake_len_this_iter = false;
    let mut delay_gap: u8 = 100;
    loop {
        if let Some(press) = consumer.dequeue() {
            // match with button
            match press.button {
                Button::Down => {
                    player.set_direction(Direction::Down);
                }
                Button::Up => {
                    player.set_direction(Direction::Up);
                }
                Button::Left => {
                    player.set_direction(Direction::Left);
                }
                Button::Right => {
                    player.set_direction(Direction::Right);
                }
                _ => {}
            }
        }

        if player.is_player_eat_food(&food) {
            food.respawn(&mut display);
            flag_incr_snake_len_this_iter = true;
            if delay_gap > 40 {
                delay_gap -= 5;
            }
        }

        if player.is_self_intersecting() {
            uled.set_high().unwrap();
            loop {} // effectively exiting...
        }

        // clear previously printed sprite
        player
            .cells_queue
            .enqueue((
                player.head_sprite.primitive.top_left.x,
                player.head_sprite.primitive.top_left.y,
            ))
            .unwrap();
        player.translate(&mut display);
        // if snake eats food, we don't clear the coord in the queue effectively
        // increasing the snake's size
        if !flag_incr_snake_len_this_iter {
            Rectangle::new(
                Point::from(player.cells_queue.dequeue().unwrap()),
                Size::new(CELL_SIZE, CELL_SIZE),
            )
            .into_styled(black_style)
            .draw(&mut display)
            .unwrap();
        }
        flag_incr_snake_len_this_iter = false;
        delay.delay_ms(delay_gap);
    }
}

struct Food<'a> {
    sprite: Styled<Rectangle, PrimitiveStyle<Rgb565>>,
    rng: Rand32,
    food_style: &'a PrimitiveStyle<Rgb565>,
    snake_style: &'a PrimitiveStyle<Rgb565>,
}

impl<'a> Food<'a> {
    fn init_and_draw<D>(
        seed: u64,
        food_style: &'a PrimitiveStyle<Rgb565>,
        snake_style: &'a PrimitiveStyle<Rgb565>,
        display: &mut D,
    ) -> Self
    where
        D: DrawTarget<Color = Rgb565>,
        <D as DrawTarget>::Error: Debug,
    {
        let mut rng = Rand32::new(seed);
        let x = (rng.rand_range(0..GRID_WIDTH) * CELL_SIZE) as i32;
        let y = (rng.rand_range(0..GRID_HEIGHT) * CELL_SIZE) as i32;
        let sprite = Rectangle::new(Point::new(x, y), Size::new(CELL_SIZE, CELL_SIZE))
            .into_styled(*food_style);
        sprite.draw(display).unwrap();
        Self {
            sprite,
            rng,
            food_style,
            snake_style,
        }
    }

    fn respawn<D>(&mut self, display: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
        <D as DrawTarget>::Error: Debug,
    {
        // clear previous food sprite
        self.sprite
            .primitive
            .draw_styled(self.snake_style, display)
            .unwrap();
        let x = (self.rng.rand_range(0..GRID_WIDTH) * CELL_SIZE) as i32;
        let y = (self.rng.rand_range(0..GRID_HEIGHT) * CELL_SIZE) as i32;
        let sprite = Rectangle::new(Point::new(x, y), Size::new(CELL_SIZE, CELL_SIZE))
            .into_styled(*self.food_style);
        self.sprite = sprite;
        self.sprite.draw(display).unwrap();
    }
}

struct Snake {
    head_sprite: Styled<Rectangle, PrimitiveStyle<Rgb565>>,
    snake_direction: Direction,
    cells_queue: Queue<(i32, i32), U64>,
}

impl Snake {
    fn init() -> Self {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::WHITE)
            .build();
        let position = Point::new(
            ((GRID_WIDTH / 2 - 1) * CELL_SIZE) as i32,
            ((GRID_HEIGHT / 2 - 1) * CELL_SIZE) as i32,
        );
        let sprite = Rectangle::new(position, Size::new(CELL_SIZE, CELL_SIZE)).into_styled(style);
        let cells_queue: Queue<(i32, i32), U64> = Queue::new();
        Self {
            head_sprite: sprite,
            snake_direction: Direction::Down,
            cells_queue,
        }
    }

    fn translate<D>(&mut self, display: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
        <D as DrawTarget>::Error: Debug,
    {
        match self.snake_direction {
            Direction::Up => self.head_sprite.primitive.top_left.y -= CELL_SIZE as i32,
            Direction::Down => self.head_sprite.primitive.top_left.y += CELL_SIZE as i32,
            Direction::Left => self.head_sprite.primitive.top_left.x -= CELL_SIZE as i32,
            Direction::Right => self.head_sprite.primitive.top_left.x += CELL_SIZE as i32,
        }

        // code for wrap-around
        if self.head_sprite.primitive.top_left.y < 0 {
            self.head_sprite.primitive.top_left.y = DISPLAY_HEIGHT as i32 - CELL_SIZE as i32;
        }

        if self.head_sprite.primitive.top_left.x < 0 {
            self.head_sprite.primitive.top_left.x = DISPLAY_WIDTH as i32 - CELL_SIZE as i32;
        }

        if self.head_sprite.primitive.top_left.x >= DISPLAY_WIDTH as i32 {
            self.head_sprite.primitive.top_left.x = 0;
        }

        if self.head_sprite.primitive.top_left.y >= DISPLAY_HEIGHT as i32 {
            self.head_sprite.primitive.top_left.y = 0;
        }
        self.head_sprite.draw(display).unwrap();
    }

    fn set_direction(&mut self, direction: Direction) {
        if self.snake_direction == Direction::Up && direction == Direction::Down {
            return;
        }
        if self.snake_direction == Direction::Down && direction == Direction::Up {
            return;
        }
        if self.snake_direction == Direction::Left && direction == Direction::Right {
            return;
        }
        if self.snake_direction == Direction::Right && direction == Direction::Left {
            return;
        }
        self.snake_direction = direction;
    }

    fn is_player_eat_food(&self, food: &Food) -> bool {
        !self
            .head_sprite
            .primitive
            .intersection(&food.sprite.primitive)
            .is_zero_sized()
    }

    fn is_self_intersecting(&self) -> bool {
        let mut result = false;
        for coord in self.cells_queue.iter() {
            result = result
                || !Rectangle::new(Point::from(*coord), Size::from((CELL_SIZE, CELL_SIZE)))
                    .intersection(&self.head_sprite.primitive)
                    .is_zero_sized()
        }
        result
    }
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
