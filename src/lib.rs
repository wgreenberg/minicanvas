use std::panic;
use wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;

mod scene;

use scene::Scene;

#[derive(Debug, Copy, Clone, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct MouseState {
    pub position: Position,
    pub left_button: ButtonState,
    pub right_button: ButtonState,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum ButtonState {
    #[default]
    Released,
    Pressed,
    Held,
}

impl From<u8> for ButtonState {
    fn from(s: u8) -> Self {
        match s {
            1 => ButtonState::Pressed,
            2 => ButtonState::Held,
            _ => ButtonState::Released,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct InputState {
    pub time: f64,
    pub frame: u32,
    pub dt: f64,
    pub mouse: MouseState,
}

trait Renderable {
    fn update(&mut self, state: InputState);
    fn render(&self, buffer: &mut [u32; WIDTH * HEIGHT]);
}

#[no_mangle]
static mut FRAME_BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
#[no_mangle]
static mut ERROR_BUFFER: [u8; 1000] = [0; 1000];

static mut SCENE: Option<Scene> = None;

fn panic_hook(info: &panic::PanicInfo) {
    let msg = info.to_string();
    // TODO: get backtrace
    let msg_bytes = msg.as_bytes();
    unsafe {
        let length = msg_bytes.len().min(ERROR_BUFFER.len());
        for i in 0..length {
            ERROR_BUFFER[i] = msg_bytes[i];
        }
    }
}

#[no_mangle]
pub unsafe extern fn init() {
    panic::set_hook(Box::new(panic_hook));
    if SCENE.is_none() {
        SCENE = Some(Scene::new());
    }
}

#[no_mangle]
pub unsafe extern fn update(time: f64, dt: f64, frame: u32, mouse_x: f64, mouse_y: f64, mouse_left_button_state: u8, mouse_right_button_state: u8) {
    let mut scene = SCENE.as_mut().unwrap();
    let state = InputState {
        time,
        dt,
        frame,
        mouse: MouseState {
            position: Position { x: mouse_x, y: mouse_y },
            left_button: mouse_left_button_state.into(),
            right_button: mouse_right_button_state.into(),
        }
    };
    scene.update(state);
}

#[no_mangle]
pub unsafe extern fn render() {
    SCENE.as_mut().unwrap().render(&mut FRAME_BUFFER);
}
