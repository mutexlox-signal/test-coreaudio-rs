//! libcubeb api/function test. Records from the microphone and plays to the speaker.
extern crate cubeb;

use cubeb::MonoFrame;
use std::thread;
use std::time::Duration;

const SAMPLE_FREQUENCY: u32 = 48_000;
const STREAM_FORMAT: cubeb::SampleFormat = cubeb::SampleFormat::S16NE;

type Frame = MonoFrame<i16>;

use cubeb::Context;

fn main() {
    let ctx = Context::init(Some(c"Cubeb recording example"), None).expect("Failed to create cubeb context");

    println!("using backend {}", ctx.backend_id());

    let params = cubeb::StreamParamsBuilder::new()
        .format(STREAM_FORMAT)
        .rate(SAMPLE_FREQUENCY)
        .channels(1)
        .layout(cubeb::ChannelLayout::MONO)
        .take();

    println!("backend {}", ctx.backend_id());

    let mut builder = cubeb::StreamBuilder::<Frame>::new();
    builder
        .name("Cubeb recording (mono)")
        .default_output(&params)
        .default_input(&params)
        .latency(0x1000)
        .data_callback(move |input, output| {
            for (i, x) in input.iter().enumerate() {
                output[i] = *x
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
