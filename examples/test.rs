//! 
//! Simple example that applies reverb to the default input device's stream and passes it straight
//! to the default output device's stream.
//!


extern crate dsp;
extern crate lanceverb;
extern crate portaudio as pa;

use dsp::{Node, Sample};
use lanceverb::Reverb;


fn main() {
    run().unwrap()
}

fn run() -> Result<(), pa::Error> {

    const CHANNELS: u16 = 2;
    const FRAMES: u32 = 128;
    const SAMPLE_HZ: f64 = 44_100.0;

    // Construct the default reverb.
    let mut verb = Reverb::new();

    // Callback used to construct the duplex sound stream.
    let callback = move |pa::DuplexStreamCallbackArgs { in_buffer, out_buffer, .. }| {
        Sample::zero_buffer(out_buffer);
        for (out_s, in_s) in out_buffer.iter_mut().zip(in_buffer.iter()) {
            *out_s = if *in_s > 1.0 { 1.0 }
                     else if *in_s < -1.0 { -1.0 }
                     else { *in_s };
        }

        let settings = dsp::Settings::new(SAMPLE_HZ as u32, FRAMES as u16, CHANNELS as u16);
        verb.audio_requested(out_buffer, settings);
        pa::Continue
    };

    // Construct PortAudio and the stream.
    let pa = try!(pa::PortAudio::new());
    let chans = CHANNELS as i32;
    let settings = try!(pa.default_duplex_stream_settings::<f32, f32>(chans, chans, SAMPLE_HZ, FRAMES));
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));
    try!(stream.start());

    // Wait for our stream to finish.
    while let Ok(true) = stream.is_active() {
        ::std::thread::sleep_ms(16);
    }

    Ok(())
}

