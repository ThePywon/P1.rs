pub mod math;
pub mod event;
pub mod audio;
pub mod ecs;

use std::sync::mpsc::Receiver;

use glfw::{Glfw, Window, WindowMode, Context, WindowEvent};
use event::event_manager::KeyOnlyEventManager;
use num_traits::ToPrimitive;
use std::mem::{size_of, size_of_val};
use ecs::{component::{Component, get_component_id}, entity::Scene};

struct A();
impl Component for A{}

struct B();
impl Component for B{}

struct C();
impl Component for C{}

#[derive(PartialEq, Eq, Hash)]
pub enum EventType {
  WindowCreate,
  WindowClose
}

#[derive(PartialEq, Eq, Hash)]
pub enum WindowEventType {
  Update
}

pub struct P1 {
  glfw: Glfw,
  windows: Vec<(Window, Receiver<(f64, WindowEvent)>, KeyOnlyEventManager<WindowEventType>)>,
  scenes: Vec<Scene>,
  pub event_manager: KeyOnlyEventManager<EventType>
}

pub fn init(glfw: Glfw) -> P1 {
  P1 { glfw, windows: Vec::new(), scenes: Vec::new(), event_manager: KeyOnlyEventManager::new() }
}

impl P1 {
  pub fn create_window(&mut self, width: u32, height: u32, title: &str, mode: WindowMode<'_>) -> (&mut Window, &mut KeyOnlyEventManager<WindowEventType>) {
    let (mut window, events) = self.glfw.create_window(width, height, title, mode)
      .expect("Failed to create GLFW window.");

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    
    window.make_current();
    window.set_key_polling(true);
    window.set_refresh_polling(true);

    self.windows.push((window, events, KeyOnlyEventManager::new()));
    self.event_manager.emit(EventType::WindowCreate);

    let last = self.windows.len() - 1;
    let (window, _, event_manager) = &mut self.windows[last];
    (window, event_manager)
  }

  pub fn run(&mut self) {
    println!("{}", get_component_id::<A>());
    println!("{}", get_component_id::<B>());
    println!("{}", get_component_id::<A>());
    println!("{}", get_component_id::<C>());
    println!("{}", get_component_id::<B>());
    println!("{}", get_component_id::<B>());
    println!("{}", get_component_id::<C>());
    println!("{}", get_component_id::<A>());

    while self.windows.len() > 0 {
      self.glfw.poll_events();

      let mut bad_windows: Vec<usize> = Vec::new();

      for (index, (window, events, event_manager)) in self.windows.iter_mut().enumerate() {
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

        window.make_current();

        let (x, y) = window.get_pos();

        unsafe {
          gl::ClearColor(x.to_f32().unwrap() / 1080f32, y.to_f32().unwrap() / 960f32, 0f32, 1f32);
          gl::Clear(gl::COLOR_BUFFER_BIT);

          let mut vao = 0;
          gl::GenVertexArrays(1, &mut vao);
          assert_ne!(vao, 0);
          gl::BindVertexArray(vao);
          let mut vbo = 0;
          gl::GenBuffers(1, &mut vbo);
          assert_ne!(vbo, 0);
          gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
          type Vertex = [f32; 3];
          const VERTICES: [Vertex; 6] = [
            [-0.5, -0.5, 0.0],
            [-0.5, 0.5, 0.0],
            [0.5, -0.5, 0.0],
            [0.5, -0.5, 0.0],
            [-0.5, 0.5, 0.0],
            [0.5, 0.5, 0.0]
          ];
          gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW
          );
          gl::VertexAttribPointer(
            0, 3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
          );
          gl::EnableVertexAttribArray(0);
          let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
          assert_ne!(vertex_shader, 0);
          const VERT_SHADER: &str = r#"#version 460 core
            layout (location = 0) in vec3 pos;
            void main() {
              gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
            }
          "#;
          gl::ShaderSource(
            vertex_shader, 1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
          );
          gl::CompileShader(vertex_shader);
          let mut success = 0;
          gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
          if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
              vertex_shader,
              1024,
              &mut log_len,
              v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
          }
          let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
          assert_ne!(fragment_shader, 0);
          const FRAG_SHADER: &str = r#"#version 330 core
            out vec4 final_color;

            void main() {
              final_color = vec4(1.0, 0.5, 0.2, 1.0);
            }
          "#;
          gl::ShaderSource(
            fragment_shader, 1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
          );
          gl::CompileShader(fragment_shader);
          let mut success = 0;
          gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
          if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
              fragment_shader,
              1024,
              &mut log_len,
              v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
          }
          let shader_program = gl::CreateProgram();
          gl::AttachShader(shader_program, vertex_shader);
          gl::AttachShader(shader_program, fragment_shader);
          gl::LinkProgram(shader_program);
          let mut success = 0;
          gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
          if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(
              shader_program,
              1024,
              &mut log_len,
              v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
          }
          gl::DeleteShader(vertex_shader);
          gl::DeleteShader(fragment_shader);
          gl::UseProgram(shader_program);
          gl::DrawArrays(gl::TRIANGLES, 0, 6);
          gl::DeleteProgram(shader_program);
        }

        event_manager.emit(WindowEventType::Update);
        
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
