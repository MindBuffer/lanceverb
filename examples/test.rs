
extern crate dsp;
extern crate lanceverb;

use dsp::{CallbackFlags, CallbackResult, Graph, Node, Sample,
          Settings, SoundStream, StreamParams, Wave};
use lanceverb::Reverb;


fn main() {

    let mut verb = Reverb::new();
    // verb.bandwidth(0.9995);
    // verb.decay(0.9);
    // verb.damping(0.3);

    let callback = Box::new(move |input: &[f32], in_settings: Settings,
                                  output: &mut[f32], out_settings: Settings,
                                  dt: f64,
                                  _: CallbackFlags| {

        Sample::zero_buffer(output);

        for (out_sample, in_sample) in output.iter_mut().zip(input.iter()) {
            //println!("{:?}", in_sample);
            *out_sample = if in_sample.abs() > 1.0 { 1.0 } else { *in_sample };
        }

        verb.audio_requested(output, out_settings);

        CallbackResult::Continue
    });

    // Construct the stream and handle any errors that may have occurred.
    let stream = SoundStream::new()
        .duplex(StreamParams::new(), StreamParams::new())
        .run_callback(callback)
        .unwrap();

    // Wait for our stream to finish.
    while let Ok(true) = stream.is_active() {
        ::std::thread::sleep_ms(16);
    }

}

