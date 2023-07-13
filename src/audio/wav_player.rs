use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpecWAV, AudioCVT};
use sdl2::Sdl;
use num_traits::ToPrimitive;
use sdl2::audio::AudioDevice;

pub struct WavPlayer<'a> {
  context: &'a Sdl,
  bytes: Vec<u8>,
  position: usize,
  pub volume: f32
}

unsafe impl<'a> Send for WavPlayer<'a> {}
unsafe impl<'a> Sync for WavPlayer<'a> {}

impl<'a> AudioCallback for &mut WavPlayer<'a> {
  type Channel = u8;

  fn callback(&mut self, data: &mut [u8]) {
    for (index, out) in data.iter_mut().enumerate() {
      let mut pos = self.position + index;

      if pos >= self.bytes.len() {
        pos %= self.bytes.len();
      }

      let mut final_output = self.bytes[pos].to_f32().unwrap() * self.volume;
      if final_output > 255f32 {
        final_output = 255f32;
      }

      *out = final_output.to_u8().unwrap();
    }

    self.position += data.len();
  }
}

impl<'a> WavPlayer<'a> {
  pub fn new(context: &'a Sdl) -> Self {
    WavPlayer { context, bytes: Vec::new(), position: 0, volume: 1f32 }
  }

  pub fn load_file(&'a mut self, filepath: &str, desired_specs: &AudioSpecDesired) -> Result<AudioDevice<&mut WavPlayer>, String> {
    let file_specs = AudioSpecWAV::load_wav(filepath)?;

    self.context.audio()?.open_playback(None, desired_specs, |specs| {
      let audio_cvt = AudioCVT::new(
        file_specs.format,
        file_specs.channels,
        file_specs.freq,
        specs.format,
        specs.channels,
        specs.freq
      );

      self.bytes = audio_cvt.unwrap().convert(file_specs.buffer().to_vec());

      self
    })
  }
}
