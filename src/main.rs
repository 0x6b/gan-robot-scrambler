mod face_rotation;

use std::iter::repeat;

use bstr::ByteSlice;
use esp32_nimble::{uuid128, BLEClient, BLEDevice, BLEScan};
use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio::{IOPin, PinDriver, Pull},
        peripherals::Peripherals,
        task::block_on,
    },
    log::EspLogger,
};
use esp_idf_sys::link_patches;
use log::info;
use smart_leds::SmartLedsWrite;
use ws2812_esp32_rmt_driver::{driver::color::LedPixelColorGrbw32, LedPixelEsp32Rmt, RGBW8};

use crate::face_rotation::{FaceRotation, FaceRotation::*};

const DEVICE_NAME: &str = "GAN-a7f13";
const SERVICE_UUID: &str = "0000fff0-0000-1000-8000-00805f9b34fb";
const CHARACTERISTIC_UUID: &str = "0000fff3-0000-1000-8000-00805f9b34fb";

const FACE_ROTATION_MAP: [FaceRotation; 20] = [
    R, R2, R2Prime, RPrime, F, F2, F2Prime, FPrime, D, D2, D2Prime, DPrime, L, L2, L2Prime, LPrime,
    B, B2, B2Prime, BPrime,
];

fn main() -> anyhow::Result<()> {
    info!("Initializing");
    link_patches();
    EspLogger::initialize_default();

    info!("Getting peripherals");
    let peripherals = Peripherals::take()?;

    info!("Setting up LED");
    let led_pin = peripherals.pins.gpio2;
    let channel = peripherals.rmt.channel0;
    let mut led = LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(channel, led_pin)?;

    info!("Setting up button");
    let mut btn_pin = PinDriver::input(peripherals.pins.gpio3.downgrade())?;
    btn_pin.set_pull(Pull::Up)?;

    info!("Starting BLE scan");
    block_on(async {
        let ble_device = BLEDevice::take();
        let mut ble_scan = BLEScan::new();
        let device = ble_scan
            .active_scan(true)
            .interval(100)
            .window(99)
            .start(ble_device, 10000, |device, data| {
                if let Some(name) = data.name() {
                    if name.contains_str(DEVICE_NAME) {
                        return Some(*device);
                    }
                }
                None
            })
            .await?;

        if let Some(device) = device {
            info!("Found device: {DEVICE_NAME}");
            let mut client = BLEClient::new();

            client.on_connect(|client| {
                client.update_conn_params(120, 120, 0, 60).unwrap();
                info!("Connected to {DEVICE_NAME}");
            });

            client.connect(&device.addr()).await?;
            let service = client.get_service(uuid128!(SERVICE_UUID)).await?;
            info!("Found service: {service}");
            let characteristic = service.get_characteristic(uuid128!(CHARACTERISTIC_UUID)).await?;
            info!("Found characteristic: {characteristic}");
            info!("Ready");

            loop {
                let pixels = RGBW8::new_alpha(0, 0, 255, smart_leds::White(128));
                led.write(repeat(pixels).take(1))?;

                if btn_pin.is_low() {
                    info!("Button pressed. Pick 8 random moves");
                    let moves = (0..8)
                        .map(|_| {
                            FACE_ROTATION_MAP[unsafe { (esp_idf_sys::esp_random() % 20) as usize }]
                        })
                        .inspect(|m| info!("- Move: {m}"))
                        .map(u8::from)
                        .collect::<Vec<_>>();
                    characteristic.write_value(&moves, false).await?;
                }

                FreeRtos::delay_ms(100);
            }
        }
        Ok(())
    })
}
