use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rodio::{Decoder, OutputStreamBuilder, Sink, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::cpal;
use std::fs;
use std::collections::HashMap;

fn list_devices() {
    let host = rodio::cpal::default_host();
    let devices = host.output_devices().expect("No output devices found");

    for device in devices {
        println!("Device name: {}", device.name().expect("Could not get device name"));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // HDA Intel PCH
    list_devices();

    let paths = fs::read_dir("/samples/").unwrap();

    let mut wav_key_map = HashMap::new();

    // get an output stream handle to the default physical sound device.
    // this keeps the audio playing in a separate thread.
    let stream_handle = OutputStreamBuilder::open_default_stream().expect("open default audio stream");

    // connect a sink to the stream's mixer. the sink manages playback.
    let sink = Sink::connect_new(&stream_handle.mixer());

    for path in paths {
        println!("Name: {}", path.as_ref().unwrap().path().display());

        // open a wav file
        let file = File::open(path.as_ref().unwrap().path()).expect("Failed to open sound file");

        // decode that sound file into a source
        let source = Decoder::try_from(file).unwrap();

        // play the sound directly on the device
        let buffered_source = source.buffered();
        wav_key_map.insert(path.as_ref().unwrap().file_name().to_str().unwrap().chars().next().unwrap(), buffered_source);

    }


    // enable raw mode to get input without pressing enter
    enable_raw_mode()?;

    // print instructions
    println!("press '?' to exit");

    loop {

        // poll for events (non-blocking check)
        if event::poll(std::time::Duration::from_millis(10))? {

            // read the event if available
            if let Event::Key(key_event) = event::read()? {

                // we only care about keypress events (not keyrelease or repeat)
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('?') => {
                            // exit the loop
                            break;                         
                        }
                        KeyCode::Char(c) => {
                            let wav_option = wav_key_map.get(&c);
                            match wav_option {
                                Some(wav) => stream_handle.mixer().add(wav.clone()),
                                None => {},
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Disable raw mode before exiting
    disable_raw_mode()?;
    Ok(())
}
