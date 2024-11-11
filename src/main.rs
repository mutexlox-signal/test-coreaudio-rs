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
        .prefs(cubeb_core::StreamPrefs::VOICE)
        .take();

    let device_id = ctx
        .enumerate_devices(cubeb::DeviceType::OUTPUT)
        .unwrap()
        .iter()
        .find(|d| d.friendly_name() == Some("BlackHole 2ch"))
        .unwrap()
        .devid();

    let mut builder = cubeb::StreamBuilder::<Frame>::new();
    builder
        .name("Cubeb recording (mono)")
        .output(device_id, &params)
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

    stream
        .set_input_processing_params(
            cubeb_core::InputProcessingParams::ECHO_CANCELLATION
                | cubeb_core::InputProcessingParams::NOISE_SUPPRESSION
                | cubeb_core::InputProcessingParams::AUTOMATIC_GAIN_CONTROL,
        )
        .expect("failed to set params");

    stream.start().unwrap();
    thread::sleep(Duration::from_millis(30000));
    stream.stop().unwrap();
}
