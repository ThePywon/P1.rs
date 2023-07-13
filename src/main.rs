use sdl2::video::GLProfile;
use sdl2::audio::AudioSpecDesired;
use p1::audio::wav_player::WavPlayer;
use p1::ecs::component;

fn on_create() {
  println!("Window created!");
}

fn on_close() {
  println!("Window closed!");
}

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let gl_attr = video_subsystem.gl_attr();

  gl_attr.set_context_profile(GLProfile::Core);
  gl_attr.set_context_version(4, 6);

  let desired_spec = AudioSpecDesired {
    freq: Some(44100),
    channels: Some(1),  // mono
    samples: None       // default sample size
  };

  let mut player = WavPlayer::new(&sdl_context);

  let device = player.load_file("assets/Berserk Skeleton Meme (Extended).wav", &desired_spec).unwrap();

  // Start playback
  device.resume();

	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

  let mut p1 = p1::init(glfw);

  p1.event_manager.on(p1::EventType::WindowCreate, on_create);
  p1.event_manager.on(p1::EventType::WindowClose, on_close);

  let (_, event_manager) = p1.create_window(300, 300, "Engine", glfw::WindowMode::Windowed);

  event_manager.on(p1::WindowEventType::Update, update);

  p1.run();
}

fn update() {
  //println!("Called!");
}
