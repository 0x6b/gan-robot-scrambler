mod face_rotation;

use std::iter::repeat;

use bstr::ByteSlice;
use esp32_nimble::{utilities::BleUuid, uuid128, BLEClient, BLEDevice, BLEScan};
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
const SERVICE_UUID: BleUuid = uuid128!("0000fff0-0000-1000-8000-00805f9b34fb");
const MOVE_CHARACTERISTIC_UUID: BleUuid = uuid128!("0000fff3-0000-1000-8000-00805f9b34fb");
const STATUS_CHARACTERISTIC_UUID: BleUuid = uuid128!("0000fff2-0000-1000-8000-00805f9b34fb");
const QUANTUM_TURN_DURATION_MS: usize = 150;
const DOUBLE_TURN_DURATION_MS: usize = 250;
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
            let service = client.get_service(SERVICE_UUID).await?;
            info!("Found service: {service}");
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

                    let mut bytes = [0u8; 18];
                    moves.iter().enumerate().for_each(|(i, &m)| {
                        let byte_index = i / 2;
                        bytes[byte_index] += m;
                        if i % 2 == 0 {
                            bytes[byte_index] *= 0x10;
                        }
                    });

                    if moves.len() % 2 == 1 {
                        bytes[(moves.len() / 2).saturating_sub(1)] += 0x0f;
                    }

                    for i in bytes.iter_mut().skip(moves.len()) {
                        *i = 0xff;
                    }

                    let sleep_duration = moves.iter().map(|&m| move_duration(m)).sum::<usize>();

                    service
                        .get_characteristic(MOVE_CHARACTERISTIC_UUID)
                        .await?
                        .write_value(&moves, false)
                        .await?;
                    FreeRtos::delay_ms((sleep_duration as f64 * 0.75) as u32);

                    while {
                        service
                            .get_characteristic(STATUS_CHARACTERISTIC_UUID)
                            .await?
                            .read_value()
                            .await?[0]
                            > 0
                    } {
                        FreeRtos::delay_ms(100);
                    }
                }
            }
        }
        Ok(())
    })
}

fn is_double_turn_move(m: u8) -> bool {
    m % 3 == 1
}

fn move_duration(m: u8) -> usize {
    if is_double_turn_move(m) {
        DOUBLE_TURN_DURATION_MS
    } else {
        QUANTUM_TURN_DURATION_MS
    }
}
