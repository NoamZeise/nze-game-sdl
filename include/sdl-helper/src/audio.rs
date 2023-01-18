//! Used for playing music and sound effects
//!
//! Make an `AudioManager` to inititalize sdl2_mixer.
//! `AudioManager` holds a struct for controlling music, and has functions for sound effect loading and playback.

use std::collections::HashMap;
use std::path::Path;
use sdl2::mixer;
use sdl2::mixer::{InitFlag, Chunk};
use crate::{Error, init_err, resource::SoundEffect, resource::Music, use_resource, unload_resource};

macro_rules! audio_load {
    (
        $(#[$($attrss:tt)*])*,
        $fn_name:ident($self:ident) -> $res_type:tt,
        $res_list:expr, $res_map: expr, $name:expr,
        $load_cmd:expr
    ) => {
        $(#[$($attrss)*])*
        pub fn $fn_name(&mut $self, filepath: &Path) -> Result<$res_type, Error> {
            let index = $crate::load_resource!(
                filepath, $res_list, $res_map, $name, Some($crate::file_err!($load_cmd(filepath))?));
            Ok($res_type{ id: index })
        }
    };
}

/// Holds audio resources and controls playback of audio
///
/// There are multiple sound effect channels, but only one
/// channel for music playback.
pub struct AudioManager<'a> {
    pub music: MusicManager<'a>,
    pub sfx: SfxManager,
    _mixer_context: mixer::Sdl2MixerContext,
}

impl<'a> AudioManager<'a> {
    /// create a new AudioManager
    ///
    /// this starts the sdl_mixer context
    pub fn new() -> Result<AudioManager<'a>, Error> {
        init_err!(mixer::open_audio(
            mixer::DEFAULT_FREQUENCY,
            mixer::DEFAULT_FORMAT,
            mixer::DEFAULT_CHANNELS,
            512, // between 256 and 1024 recommened, lower number = lower latency but not as compatible
        ))?;

        let _mixer_context = init_err!(mixer::init(InitFlag::all()))?;

        Ok(AudioManager {
            _mixer_context,
            sfx: SfxManager::new(),
            music : MusicManager::new(),
        })
    }
}

/// Load and play [resource::SoundEffect], created and owned by [AudioManager]
pub struct SfxManager {
    sound_effects: Vec<Option<Chunk>>,
    sound_effects_paths: HashMap<String, usize>,
}

impl SfxManager {
    fn new() -> SfxManager {
        SfxManager { sound_effects: Vec::new(), sound_effects_paths: HashMap::new() }
    }
    
    audio_load!(
        /// load sound effect to memory
        , load(self) -> SoundEffect,
        self.sound_effects, self.sound_effects_paths, "Sound Effect",
        Chunk::from_file
    );

    fn get_sfx(&mut self, sfx: SoundEffect) -> Result<&mut Chunk, Error> {
        use_resource!(
            self.sound_effects, sfx.id,
            Some(s) => {
                Ok(s)
            }   
        )
    }

    /// Plays the sound effect
    pub fn play(&mut self, sfx: SoundEffect) -> Result<(), Error> {
        mixer::Channel::all().play(self.get_sfx(sfx)?, 1)
            .map_err(
                |e| Error::AudioPlay("failed to play sound effect, sdl_mixer error: ".to_string() + &e)
            )?;
        Ok(())
    }

    unload_resource!(
        /// unloades the internal `Sound Effect`
        , unload, self, self.sound_effects_paths, self.sound_effects, sfx, SoundEffect, "Sound Effect");

    /// Set the volume of the sound effect
    ///
    /// returns error if the resource could not be found
    ///
    /// the range of volume values is `0.0` to `1.0`
    pub fn set_volume(&mut self, sfx: SoundEffect, volume: f64) -> Result<(), Error> {
        self.get_sfx(sfx)?.set_volume((volume * 128.0) as i32);
        Ok(())
    }

    /// Get the volume of the sound effect
    ///
    /// returns error if the resource could not be found
    ///
    /// the range of volume values is `0.0` to `1.0`
    pub fn get_volume(&mut self, sfx: SoundEffect) -> Result<f64, Error> {
        Ok(self.get_sfx(sfx)?.get_volume() as f64 / 128.0)
    }
}

/// Load and play [resource::Music], created and owned by [AudioManager]
pub struct MusicManager<'a> {
    music : Vec<Option<mixer::Music<'a>>>,
    music_paths: HashMap<String, usize>,
}

impl<'a> MusicManager<'a> {
    fn new() -> MusicManager<'a> {
        MusicManager { music: Vec::new(), music_paths: HashMap::new()}
    }

    audio_load!(
        /// load `Music` to memory
        , load(self) -> Music,
        self.music, self.music_paths, "Music",
        sdl2::mixer::Music::from_file
    );

    /// Play the music file
    ///
    /// - the repeat value will loop continously if you pass `-1`
    /// - volume is on a scale from `0.0` to `1.0`
    pub fn play(&mut self, music: Music, repeats: i32) -> Result<(), Error> {
        use_resource!(
            self.music, music.id,
            Some(s) => {
                s.play(repeats).map_err(
                    |e| Error::AudioPlay("failed to play music, sdl_mixer error: ".to_string() + &e)
                )?;
                Ok(())
            }   
        )
    }
    
    unload_resource!(
        /// unloades the internal `Music`
        , unload, self, self.music_paths, self.music, music, Music, "Music");

    /// Returns true if there is music currently playing
    pub fn playing(&self) -> bool {
        mixer::Music::is_playing()
    }

    /// Returns true if there is music in the music channel that is paused
    pub fn paused(&self) -> bool {
        mixer::Music::is_paused()
    }

    /// set the current volume of the music channel
    ///
    /// the range of values is `0.0` to `1.0`
    pub fn set_volume(&self, volume: f64) {
        mixer::Music::set_volume((volume * 128.0) as i32);
    }

    /// get the current volume of the music channel
    ///
    /// the range of values is `0.0` to `1.0`
    pub fn get_volume(&self) -> f64 {
        mixer::Music::get_volume() as f64 / 128.0
    }

    /// Pauses the currently playing music
    ///
    /// does nothing if no music is playing
    pub fn pause(&self) {
        mixer::Music::pause();
    }

    /// Resume paused music
    ///
    /// does nothing if no music has been paused
    pub fn resume(&self) {
        mixer::Music::resume();
    }

    //TODO add music fade functions
}
