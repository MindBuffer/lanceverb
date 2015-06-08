//! 
//! Simple example that applies reverb to the default input device's stream and passes it straight
//! to the default output device's stream.
//!


extern crate dsp;
extern crate lanceverb;

use dsp::{CallbackFlags, CallbackResult, Node, Sample, Settings, SoundStream, StreamParams};
use lanceverb::Reverb;


fn main() {

    // Construct the default reverb.
    let mut verb = Reverb::new();

    // Callback used to construct the duplex sound stream.
    let callback = Box::new(move |input: &[f32], _in_settings: Settings,
                                  output: &mut[f32], out_settings: Settings,
                                  _dt: f64,
                                  _: CallbackFlags| {
        Sample::zero_buffer(output);
        for (out_s, in_s) in output.iter_mut().zip(input.iter()) {
            *out_s = if *in_s > 1.0 { 1.0 }
                     else if *in_s < -1.0 { -1.0 }
                     else { *in_s };
        }

        verb.audio_requested(output, out_settings);
        CallbackResult::Continue
    });

    // Construct the stream with default parameters.
    let stream = SoundStream::new()
        .duplex(StreamParams::new(), StreamParams::new())
        .run_callback(callback)
        .unwrap();

    // Wait for our stream to finish.
    while let Ok(true) = stream.is_active() {
        ::std::thread::sleep_ms(16);
    }

}

