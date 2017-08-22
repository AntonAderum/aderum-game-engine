extern crate sdl2;

use std::path::Path;
use sdl2::mixer::{DEFAULT_CHANNELS, INIT_MP3, INIT_FLAC, INIT_MOD, INIT_FLUIDSYNTH, INIT_MODPLUG,
                  INIT_OGG, AUDIO_S16LSB};

pub fn play(sound_file: &Path) {
    let sdl = sdl2::init().unwrap();
    let _audio = sdl.audio().unwrap();
    let mut timer = sdl.timer().unwrap();
    let _mixer_context = sdl2::mixer::init(INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_FLUIDSYNTH |
                                           INIT_MODPLUG | INIT_OGG).unwrap();

    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

    // Number of mixing channels available for sound effect `Chunk`s to play
    // simultaneously.
    sdl2::mixer::allocate_channels(4);

    let sound_chunk_res = sdl2::mixer::Chunk::from_file(sound_file);

    match sound_chunk_res {
        Ok(sound_chunk) => {
            let play_res = match sdl2::mixer::Channel::all().play(&sound_chunk, 0) {
                Ok(s) => s,
                Err(e) => {
                    panic!("{:?}", e);
                }
            };
            while play_res.is_playing() {
                timer.delay(10);
            }
        }
        Err(e) => println!("Cannot load sound file: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}