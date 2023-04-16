pub mod math;
pub mod event;

use std::sync::mpsc::Receiver;

use glfw::{Glfw, Window, WindowMode, Context, WindowEvent};
use event::event_manager::KeyOnlyEventManager;

#[derive(PartialEq, Eq, Hash)]
pub enum EventType {
  WindowCreate,
  WindowClose
}

pub struct P1 {
  glfw: Glfw,
  windows: Vec<(Window, Receiver<(f64, WindowEvent)>)>,
  pub event_manager: KeyOnlyEventManager<EventType>
}

pub fn init(glfw: Glfw) -> P1 {
  P1 { glfw, windows: Vec::new(), event_manager: KeyOnlyEventManager::new() }
}

impl P1 {
  pub fn create_window(&mut self, width: u32, height: u32, title: &str, mode: WindowMode<'_>) {
    let (mut window, events) = self.glfw.create_window(width, height, title, mode)
      .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    self.windows.push((window, events));
    self.event_manager.emit(EventType::WindowCreate);
  }

  pub fn run(&mut self) {
    while self.windows.len() > 0 {
      self.glfw.poll_events();

      let mut bad_windows: Vec<usize> = Vec::new();

      for (index, (window, events)) in self.windows.iter_mut().enumerate() {
        if window.should_close() {
          bad_windows.push(index);
          continue
        }

        for (_, event) in glfw::flush_messages(events) {
          match event {
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
              window.set_should_close(true);
            },
            _ => {}
          }
        }

        window.swap_buffers();
      }

      if bad_windows.len() > 0 {
        for bad_window in bad_windows {
          self.windows.remove(bad_window);
          self.event_manager.emit(EventType::WindowClose);
        }
      }
    }
  }
}
