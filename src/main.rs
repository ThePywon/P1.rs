use sdl2::video::GLProfile;
use sdl2::audio::AudioSpecDesired;
use p1::audio::wav_player::WavPlayer;
#[allow(unused_imports)]
use p1::ecs::scene::Scene;

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

  p1.event_manager.on(p1::EventType::WindowCreate, |_| {
    println!("Window created!");
  });
  p1.event_manager.on(p1::EventType::WindowClose, |_| {
    println!("Window closed!");
  });

  let (_, event_manager) = p1.create_window(300, 300, "Moist", glfw::WindowMode::Windowed);

  event_manager.on(p1::WindowEventType::Update, |_| {
    
  });

  let scene_id = p1.register_scene(Scene::new());
  let scene = p1.get_scene(scene_id).unwrap();

  let entity_a = scene.create_entity();

  println!("{}", entity_a);

  p1.run();
}
