use sdl2::{
    AudioSubsystem,
    audio::{AudioCallback, AudioSpecDesired, AudioDevice},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayingStatus {
    Playing,
    Stopped,
}

pub struct Bell {
    status: PlayingStatus,
    beeper: AudioDevice<Beeper>,
}

#[derive(Debug)]
pub struct Beeper {
    pub phase_inc: f32,
    pub phase: f32,
    pub volume: f32,
}

impl AudioCallback for Beeper {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

impl Bell {
    pub fn new(audio_subsystem: &AudioSubsystem) -> Self {
        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            Beeper {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.15,
            }
        }).unwrap();

        Self { status: PlayingStatus::Stopped, beeper: device }
    }

    pub fn set_status(&mut self, status: PlayingStatus) {
        self.status = status;
        match status {
            PlayingStatus::Playing => self.beeper.resume(),
            PlayingStatus::Stopped => self.beeper.pause()
        }
    }

    pub fn get_status(&self) -> PlayingStatus {
        self.status
    }

    pub fn inner_mut(&mut self) -> &mut AudioDevice<Beeper> {
        &mut self.beeper
    }
}
