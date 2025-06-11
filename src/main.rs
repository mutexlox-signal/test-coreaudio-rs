//! libcubeb api/function test. Plays a tone to a blackhole 16ch device

// based on
// https://github.com/mozilla/cubeb-rs/blob/master/cubeb-api/examples/tone.rs
// but with additional logging
extern crate cubeb;

use cubeb::{MonoFrame, Sample};
use std::f32::consts::PI;
use std::thread;
use std::time::Duration;

use cubeb::Context;

const SAMPLE_FREQUENCY: u32 = 48_000;
const STREAM_FORMAT: cubeb::SampleFormat = cubeb::SampleFormat::S16NE;

type Frame = MonoFrame<i16>;

fn main() {
    cubeb_core::set_logging(
        cubeb::LogLevel::Normal,
        Some(|s| {
            println!("{}", s.to_str().unwrap().trim());
        }),
    )
    .expect("log failed");
    let ctx = Context::init(Some(c"Cubeb recording example"), None)
        .expect("Failed to create cubeb context");

    println!("using backend {}", ctx.backend_id());

    let params = cubeb::StreamParamsBuilder::new()
        .format(STREAM_FORMAT)
        .rate(SAMPLE_FREQUENCY)
        .channels(1)
        .layout(cubeb::ChannelLayout::MONO)
        .take();

    let mut position = 0u32;
    let mut builder = cubeb::StreamBuilder::<Frame>::new();
    builder
        .name("Cubeb recording (mono)")
        .default_output(&params)
        .latency(0x1000)
        .data_callback(move |_, output| {
            // generate our test tone on the fly
            for f in output.iter_mut() {
                // North American dial tone
                let t1 = (2.0 * PI * 350.0 * position as f32 / SAMPLE_FREQUENCY as f32).sin();
                let t2 = (2.0 * PI * 440.0 * position as f32 / SAMPLE_FREQUENCY as f32).sin();

                f.m = i16::from_float(0.25 * (t1 + t2));

                position += 1;
            }
            output.len() as isize
        })
        .state_callback(|state| {
            println!("stream {:?}", state);
        });

    let stream = builder.init(&ctx).expect("Failed to create cubeb stream");

    stream.start().unwrap();
    thread::sleep(Duration::from_millis(5000));
    stream.stop().unwrap();
}
