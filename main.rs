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

/*
 * use rodio::{Decoder, OutputStream, Sink, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn main() {
    // 2. Connect a Sink to the stream's mixer. The Sink manages playback.
    let sink = Sink::connect_new(&stream_handle.mixer());

    // 3. Load the WAV file.
    // Make sure "your_sound.wav" is in the same directory or provide the full path.
    let file = File::open("your_sound.wav").expect("Failed to open sound file");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

    // 4. Add the sound source to the sink to start playback.
    sink.append(source);

    // 5. Keep the main thread alive until the sound finishes playing.
    // The sound plays in a background thread managed by rodio.
    println!("Playing sound...");
    sink.sleep_until_end(); // Blocks until all appended sounds are done.
    println!("Playback finished.");

    // Alternatively, to just keep the program running for a duration:
    // thread::sleep(Duration::from_secs(5)); // Sleep for 5 seconds
    // stream_handle.stop(); // Manually stop if needed, though dropping it stops playback.
}
*/

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

    // get an output stream handle to the default physical sound device.
    // this keeps the audio playing in a separate thread.
    let stream_handle = OutputStreamBuilder::open_default_stream().expect("open default audio stream");

    // connect a sink to the stream's mixer. the sink manages playback.
    let sink = Sink::connect_new(&stream_handle.mixer());

    // open a wav file
    let file = File::open("/samples/Phonk_/Kicks/Kick (1).wav").expect("Failed to open sound file");

    // decode that sound file into a source
    let source = Decoder::try_from(file).unwrap();

    // play the sound directly on the device
    let buffered_source = source.buffered();
    stream_handle.mixer().add(buffered_source.clone());

    // enable raw mode to get input without pressing enter
    enable_raw_mode()?;

    // print instructions
    println!("press 'q' to exit");

    loop {

        // poll for events (non-blocking check)
        if event::poll(std::time::Duration::from_millis(10))? {

            // read the event if available
            if let Event::Key(key_event) = event::read()? {

                // we only care about keypress events (not keyrelease or repeat)
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            // exit the loop
                            break;                         
                        }
                        KeyCode::Up => {
                            println!("Up arrow pressed!");
                        }
                        KeyCode::Down => {
                            println!("Down arrow pressed!");
                        }
                        KeyCode::Char(c) => {
                            println!("Key '{}' pressed!", c);
                            stream_handle.mixer().add(buffered_source.clone());
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
