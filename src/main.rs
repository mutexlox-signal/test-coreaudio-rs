//! libcubeb api/function test. Plays a tone to a blackhole 16ch device
extern crate cubeb;

use cubeb::{MonoFrame, Sample};
use std::f32::consts::PI;
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
    let ctx = Context::init(Some(c"Cubeb recording example"), Some("audiounit"))
        .expect("Failed to create cubeb context");

    println!("using backend {}", ctx.backend_id());

    for info in ctx
        .enumerate_devices(cubeb::DeviceType::INPUT)
        .unwrap()
        .iter()
        .find(|d| d.friendly_name().unwrap().contains("Blackhole 16ch"))
        .unwrap()
        .devid();

    let mut position = 0u32;
    let mut builder = cubeb::StreamBuilder::<Frame>::new();
    builder
        .name("Cubeb recording (mono)")
        .output(device_id, &params)
        .latency(0x1000)
        .data_callback(move |_, output| {
            // generate our test tone on the fly
            for f in output.iter_mut() {
                // North American dial tone
                let t1 = (2.0 * PI * 350.0 * position as f32 / SAMPLE_FREQUENCY as f32).sin();
                let t2 = (2.0 * PI * 440.0 * position as f32 / SAMPLE_FREQUENCY as f32).sin();

                f.m = i16::from_float(0.5 * (t1 + t2));

                position += 1;
            }
            output.len() as isize
        })
        .state_callback(|state| {
            println!("stream {:?}", state);
        });

    let stream = builder.init(&ctx).expect("Failed to create cubeb stream");

    stream.start().unwrap();
    thread::sleep(Duration::from_millis(30000));
    stream.stop().unwrap();
}
