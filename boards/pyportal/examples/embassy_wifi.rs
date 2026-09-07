#![no_std]
#![no_main]

use bsp::{hal, pac};
use embassy_time::Duration;
use hal::{
    clock::v2::{clock_system_at_reset, osculp32k::OscUlp32k, pclk::Pclk, rtcosc::RtcOsc},
    eic,
    sercom::{spi, Sercom2},
    time::Hertz,
};
use pyportal as bsp;
use reqwless::request::RequestBuilder;

hal::embassy_time!(Driver);

hal::bind_multiple_interrupts!(struct SercomIrq {
    SERCOM2: [SERCOM2_0, SERCOM2_1, SERCOM2_2, SERCOM2_OTHER] => spi::InterruptHandler<Sercom2>;
});

hal::bind_interrupts!(struct EicIrq {
    EIC_EXTINT_0 => eic::InterruptHandler;
});

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    defmt::println!("Hello world!");

    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();
    let pins = bsp::Pins::new(peripherals.port);

    // Select the 32khz source
    let (mut buses, clocks, tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );

    let (osculp32k, _) = OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);
    let (rtc, _) = RtcOsc::enable(tokens.rtcosc, osculp32k);

    // SAFETY: not in a critical section

    unsafe {
        Driver::init(rtc);
    }

    let _spi_clk = buses.apb.enable(tokens.apbs.sercom2);
    let (sercom2_pclk, gclk) = Pclk::enable(tokens.pclks.sercom2, clocks.gclk0);

    let (_, _, _, mut mclk) = unsafe { clocks.pac.steal() };

    let baud = Hertz::Hz(115_200);

    let (miso, mosi, sclk): (pyportal::Miso, pyportal::Mosi, pyportal::Sck) =
        (pins.miso.into(), pins.mosi.into(), pins.sck.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);
    let spi = spi::Config::new(&mclk, peripherals.sercom2, pads, sercom2_pclk.freq())
        .baud(baud.into())
        .spi_mode(spi::MODE_0)
        .enable();

    let (eicclk, _gclk) = Pclk::enable(tokens.pclks.eic, gclk);

    let eic = hal::eic::Eic::new(&mut mclk, &(eicclk.into()), peripherals.eic).split();

    defmt::println!("trying to init wifi");

    // warm_up(&mut red_led).await;

    let mut nina = pyportal::wifi(
        spi,
        SercomIrq,
        eic.0,
        pins.esp_cs,
        pins.esp_busy,
        EicIrq,
        pins.esp_reset,
        pins.esp_gpio0,
    )
    .await
    .expect("couldn't init wifi");

    defmt::println!("wifi awake");

    let mut version = [0_u8; 32];

    defmt::expect!(nina.get_fw_version(&mut version).await);
    defmt::println!(
        "Firmware version is {=str}",
        str::from_utf8(&version).expect("not a valid string")
    );

    defmt::println!("sending credentials");
    defmt::unwrap!(
        nina.connect_wpa(
            env!("WIFI_NETWORK").as_bytes(),
            env!("WIFI_PASSWORD").as_bytes()
        )
        .await
    );
    defmt::println!("waiting for connection");
    defmt::unwrap!(nina.wait_for_connected(Duration::from_secs(10)).await);
    defmt::println!("connected!");

    let mutex = embassy_sync::mutex::Mutex::<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        _,
    >::new(nina);
    let stack = embassy_nina::nal::NinaStack::new(&mutex);

    let client = reqwless::client::HttpClient::new(&stack, &stack);

    let mut buf = [0_u8; 1024];
    let result = defmt::unwrap!(get_ip(client, &mut buf).await);

    // // loop {
    let string = str::from_utf8(&result).unwrap();
    defmt::println!("IP: {}", string);
    // // }
}

async fn get_ip<'a, T, D>(
    mut client: reqwless::client::HttpClient<'a, T, D>,
    buf: &'a mut [u8],
) -> Result<&'a mut [u8], reqwless::Error>
where
    T: embedded_nal_async::TcpConnect + 'a,
    D: embedded_nal_async::Dns + 'a,
{
    client
        .request(reqwless::request::Method::GET, "http://jsonip.com")
        .await?
        .content_type(reqwless::headers::ContentType::ApplicationJson)
        .host("jsonip.com")
        .send(buf)
        .await?
        .body()
        .read_to_end()
        .await
}
