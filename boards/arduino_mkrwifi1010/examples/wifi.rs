// This connects to WiFI through SPI to the NINA-W102 Wi-Fi module using the wifi_nina library.
// This works when the NINA-W102 runs the Arduino nina-fw firmware which is installed by default
// on all Arduino MKR 1010s: https://github.com/arduino/nina-fw
// The example then prints out various debug messages to the USB serial

#![no_std]
#![no_main]

use arduino_mkrwifi1010 as bsp;
use atsamd_hal::thumbv6m::usb::UsbBus;
use bsp::hal;
use core::time::Duration;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::time::MegaHertz;

use core::fmt::Write;
use cortex_m::asm::delay as cycle_delay;
use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;
use heapless::String;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use wifi_nina::transport::SpiTransport;
use wifi_nina::types::{
    Config, ConnectionState, NetworkConfig, ProtocolMode, StationConfig, TcpState,
};

const BOOT_DELAY_MS: u16 = 100;

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

const WIFI_SSID: &[u8] = b"CHANGE-ME";
const WIFI_PASSWORD: &[u8] = b"CHANGE-ME";

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // Setup USB
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x2222, 0x3333))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    delay.delay_ms(BOOT_DELAY_MS);

    // Setup Wifi module
    let spi = bsp::nina_spi_master(
        &mut clocks,
        MegaHertz(8),
        peripherals.SERCOM2,
        &mut peripherals.PM,
        pins.nina_sck,
        pins.nina_mosi,
        pins.nina_miso,
    );

    let spi_transport = SpiTransport::start(
        spi,
        pins.nina_ack.into_floating_input(),
        pins.nina_resetn.into_push_pull_output(),
        pins.nina_cs.into_push_pull_output(),
        |d: Duration| delay.delay_ms(d.as_millis() as u32),
    )
    .unwrap();
    let mut wifi = wifi_nina::Wifi::new(spi_transport);

    // Try to connect to WiFi and then make raw HTTP request towards an IP address of http://ifconfig.co
    usbserial_write!("Connecting to WiFi\r\n");
    let configure_result = wifi.configure(
        Config::Station(StationConfig {
            network: NetworkConfig::Password {
                ssid: WIFI_SSID,
                password: WIFI_PASSWORD,
            },
        }),
        Some(Duration::from_secs(5)),
    );
    usbserial_write!(
        "Result after trying to connect for at maximum 30 seconds: {:?}\r\n",
        &configure_result
    );
    if configure_result.is_err() {
        usbserial_write!("Make sure that you specified the right SSID and password for the WiFi and that it's 2.4GHz and not 5GHz since the NINA-W102 WiFi module only supports 2.4GHz.");
    }

    let mut client_result = wifi.new_client();
    if let Ok(client) = &mut client_result {
        // We'll make a request directly to an IP of http://ifconfig.co/country (IP found by doing nslookup ifconfig.co on 2022-05-04, hopefully it continues to work.)
        // An alternative is to add DNS functionality to wifi_nina which has partial support for it implemented, but I haven't tried that
        let connection_result = client.connect_ipv4(
            &mut wifi,
            no_std_net::Ipv4Addr::new(188, 114, 96, 2),
            80,
            ProtocolMode::Tcp,
        );
        usbserial_write!("Tcp connection result: {:?}\r\n", connection_result);

        let send_result = client.send(
            &mut wifi,
            b"GET /country HTTP/1.1\nHost: ifconfig.co\nConnection: close\n\n",
        );
        usbserial_write!("Send result: {:?}\r\n", send_result);
        usbserial_write!("You are (maybe) in country (look after the headers down below):\r\n");

        while let Ok(state) = client.state(&mut wifi) {
            let mut buf = [0u8; 2048];

            let recv_result = client.recv(&mut wifi, &mut buf);
            match recv_result {
                Ok(received_bytes) => {
                    cortex_m::interrupt::free(|_| unsafe {
                        if USB_BUS.as_mut().is_some() {
                            if let Some(serial) = USB_SERIAL.as_mut() {
                                let mut write_offset = 0;
                                while write_offset < received_bytes {
                                    match serial.write(&buf[write_offset..received_bytes]) {
                                        Ok(len) if len > 0 => {
                                            write_offset += len;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    usbserial_write!("Error trying to receive data: {:?}\r\n", e);
                }
            }
            if state == TcpState::Closed {
                usbserial_write!("\r\nBreaking since TcpState is Closed.\r\n");
                break;
            }
        }
    }

    usbserial_write!("Connection status of WiFi (after HTTP connection closed above):\r\n");

    loop {
        // Since our other delay moved into SpiTransport cycle_delay was the easiest way to slow down the loop a bit
        cycle_delay(30 * 1024 * 1024);

        let connection_state =
            wifi.await_connection_state(ConnectionState::Connected, Duration::from_millis(0));
        usbserial_write!("WiFi connection state: {:?}\r\n", connection_state);
    }
}

fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.as_mut() {
            if let Some(serial) = USB_SERIAL.as_mut() {
                usb_dev.poll(&mut [serial]);

                // Make the other side happy
                let mut buf = [0u8; 16];
                let _ = serial.read(&mut buf);
            }
        }
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}

#[macro_export]
macro_rules! usbserial_write {
    ($($tt:tt)*) => {{
        let mut s: String<1024> = String::new();
        write!(s, $($tt)*).unwrap();
        let message_bytes = s.as_bytes();
        let mut total_written = 0;
        while total_written < message_bytes.len() {
            let bytes_written = disable_interrupts(|_| unsafe {
                match USB_SERIAL.as_mut().unwrap().write(
                    &message_bytes[total_written..]
                ) {
                    Ok(count) => count,
                    Err(_) => 0,
                }
            });
            total_written += bytes_written;
        }
    }};
}
