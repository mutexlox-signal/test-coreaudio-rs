//! libcubeb api/function test. Records from the microphone and plays to the speaker.
extern crate cubeb;

use std::thread;
use std::time::Duration;

use cubeb::Context;

fn main() {
    cubeb_core::set_logging(
        cubeb::LogLevel::Normal,
        Some(|s| {
            println!("{}", s.to_str().unwrap().trim());
        }),
    )
    .expect("log failed");
    let ctx = Context::init(Some(c"Cubeb enumeration  example"), None)
        .expect("Failed to create cubeb context");

    println!("using backend {}", ctx.backend_id());

    for info in ctx
        .enumerate_devices(cubeb::DeviceType::INPUT)
        .unwrap()
        .iter()
    {
        println!(
            "input device: {:?} {:?} {:?}",
            info.devid(),
            info.device_id(),
            info.friendly_name()
        );
    }

    thread::sleep(Duration::from_millis(30_000));

    for info in ctx
        .enumerate_devices(cubeb::DeviceType::INPUT)
        .unwrap()
        .iter()
    {
        println!(
            "input device: {:?} {:?} {:?}",
            info.devid(),
            info.device_id(),
            info.friendly_name()
        );
    }
}
