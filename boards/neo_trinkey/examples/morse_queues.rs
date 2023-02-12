// const INTERVAL: u16 = 100u16; // Controls the speed of morse code generation

mod morse_queues {
    const CHAR_QUEUE_LENTGH: usize = 1024;
    const STATE_QUEUE_LENGTH: usize = 32;

    struct CharQueue {
        queue: [char; CHAR_QUEUE_LENTGH],
        length: usize,
        write_position: usize,
        read_position: usize,
    }

    struct PinControlQueue {
        queue: [PinControlDescriptor; STATE_QUEUE_LENGTH],
        length: usize,
        write_position: usize,
        read_position: usize,
    }

    static mut CHAR_QUEUE: CharQueue = CharQueue {
        queue: ['\r'; CHAR_QUEUE_LENTGH],
        length: 0,
        write_position: 0,
        read_position: 0,
    };

    static mut PIN_CONTROL_QUEUE: PinControlQueue = PinControlQueue {
        queue: [
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
            PinControlDescriptor {
                pin_state: false,
                duration: 0,
            },
        ],
        length: 0,
        write_position: 0,
        read_position: 0,
    };

    struct PinControlDescriptor {
        pin_state: bool,
        duration: u8,
    }

    fn push_dot() {
        let dot = PinControlDescriptor {
            pin_state: true,
            duration: 1,
        };
        let interval = PinControlDescriptor {
            pin_state: false,
            duration: 1,
        };
        push_state(dot);
        push_state(interval);
    }

    fn push_dash() {
        let dash = PinControlDescriptor {
            pin_state: true,
            duration: 3,
        };
        let interval = PinControlDescriptor {
            pin_state: false,
            duration: 1,
        };
        push_state(dash);
        push_state(interval);
    }

    fn push_space() {
        let space = PinControlDescriptor {
            pin_state: false,
            duration: 7,
        };
        push_state(space);
    }

    fn push_letter_interval() {
        let space = PinControlDescriptor {
            pin_state: false,
            duration: 3,
        };
        push_state(space);
    }

    static mut COUNT_DOWN: u8 = 0;
    static mut CURRENT_PIN_STATE: bool = false;

    fn get_next_state() -> bool {
        let mut pin_state: bool;
        unsafe {
            if COUNT_DOWN <= 0 {
                if PIN_CONTROL_QUEUE.length == 0 {
                    // Only push a new character into our que if we've drained it fully
                    emit_morse_letter(pop_char());
                }
                let pin_state_descriptor = pop_state();
                COUNT_DOWN = pin_state_descriptor.duration;
                CURRENT_PIN_STATE = pin_state_descriptor.pin_state;
            }
            COUNT_DOWN -= 1;
            return CURRENT_PIN_STATE;
        }
    }

    fn push_char(letter: char) {
        unsafe {
            let index = CHAR_QUEUE.write_position as usize;
            let queue = &mut CHAR_QUEUE;
            queue.queue[index] = letter;
            CHAR_QUEUE.write_position = (CHAR_QUEUE.write_position + 1) % CHAR_QUEUE_LENTGH;
            queue.length += 1;
        }
    }

    fn pop_char() -> char {
        unsafe {
            if CHAR_QUEUE.length == 0 {
                return '\r'; // Use CR to indicate empty queue
            }
            let return_value = CHAR_QUEUE.queue[CHAR_QUEUE.read_position];
            CHAR_QUEUE.read_position = (CHAR_QUEUE.read_position + 1) % CHAR_QUEUE_LENTGH;
            CHAR_QUEUE.length -= 1;
            return return_value;
        }
    }

    fn push_state(state: PinControlDescriptor) {
        unsafe {
            let index = PIN_CONTROL_QUEUE.write_position as usize;
            let queue = &mut PIN_CONTROL_QUEUE;
            queue.queue[index] = state;
            PIN_CONTROL_QUEUE.write_position =
                (PIN_CONTROL_QUEUE.write_position + 1) % STATE_QUEUE_LENGTH;
            queue.length += 1;
        }
    }

    fn pop_state() -> PinControlDescriptor {
        unsafe {
            if PIN_CONTROL_QUEUE.length == 0 {
                return PinControlDescriptor {
                    pin_state: false,
                    duration: 1,
                };
            }
            let return_value = &PIN_CONTROL_QUEUE.queue[PIN_CONTROL_QUEUE.read_position];
            PIN_CONTROL_QUEUE.length -= 1;
            PIN_CONTROL_QUEUE.read_position =
                (PIN_CONTROL_QUEUE.read_position + 1) % STATE_QUEUE_LENGTH;
            return PinControlDescriptor {
                pin_state: return_value.pin_state,
                duration: return_value.duration,
            };
        }
    }

    fn emit_morse_letter(letter: char) {
        let downcased_letter = letter.to_ascii_lowercase(); // Add support for Latin 1 later.
        match downcased_letter {
            'a' => {
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            'b' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            'c' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            'd' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            'e' => {
                emit_morse_dot();
                push_letter_interval();
            }
            'f' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            'g' => {
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            'h' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            'i' => {
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            'j' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            'k' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            'l' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            'm' => {
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            'n' => {
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            'o' => {
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            'p' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            'q' => {
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            'r' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            's' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            't' => {
                emit_morse_dash();
                push_letter_interval();
            }
            'u' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            'v' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            'w' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            'x' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            'y' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            'z' => {
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            '0' => {
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            '1' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            '2' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            '3' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            '4' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            '5' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            '6' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            '7' => {
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            '8' => {
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            '9' => {
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            ' ' => {
                push_space();
            }
            ',' => {
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            '.' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            '?' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            '\'' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            '!' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                push_letter_interval();
            }
            '/' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            '(' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            ')' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            '&' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            ':' => {
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                push_letter_interval();
            }
            ';' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            '=' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            '+' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            '-' => {
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            '_' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                push_letter_interval();
            }
            '"' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            '$' => {
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            '@' => {
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dash();
                emit_morse_dot();
                emit_morse_dash();
                emit_morse_dot();
                push_letter_interval();
            }
            '\r' => {
                // We use CR to indicate the quueue is empty, so we make it phs only one blank cycle.
                push_letter_interval();
            }
            _ => {
                emit_morse_space();
            }
        }
    }

    fn emit_morse_dot() {
        push_dot();
    }
    fn emit_morse_dash() {
        push_dash();
    }
    fn emit_morse_space() {
        push_space();
    }
}
