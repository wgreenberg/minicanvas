use std::panic;
use wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;

mod rng;
mod scene;

use scene::Scene;

#[derive(Debug, Copy, Clone, Default)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    fn to_u32(&self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        let a = (self.a * 255.0) as u32;
        r | g << 8 | b << 16 | a << 24
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    fn to_px(&self) -> Option<Pixel> {
        let x = self.x + (WIDTH as f32) / 2.0;
        let y = (HEIGHT as f32) / 2.0 - self.y;
        if x < 0.0 || y < 0.0 || x >= WIDTH as f32 || y >= HEIGHT as f32 {
            None
        } else {
            Some(Pixel {
                x: x as u32,
                y: y as u32,
            })
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

pub struct Pixel {
    pub x: u32,
    pub y: u32,
}

impl Pixel {
    fn new(x: u32, y: u32) -> Pixel {
        Pixel { x, y }
    }

    fn to_index(&self) -> usize {
        self.y as usize * WIDTH + self.x as usize
    }

    fn to_pos(&self) -> Vec2 {
        let x = self.x as f32 - WIDTH as f32 / 2.0;
        let y = HEIGHT as f32 / 2.0 - self.y as f32;
        Vec2 { x, y }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct MouseState {
    pub position: Vec2,
    pub left_button: ButtonState,
    pub right_button: ButtonState,
}

#[derive(Debug, Copy, Clone)]
pub enum ButtonState {
    Released,
    Pressed,
}

impl From<u8> for ButtonState {
    fn from(s: u8) -> Self {
        match s {
            1 => ButtonState::Pressed,
            _ => ButtonState::Released,
        }
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Released
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct InputState {
    pub time: f32,
    pub frame: u32,
    pub dt: f32,
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
pub unsafe extern fn update(time: f32, dt: f32, frame: u32, mouse_x: u32, mouse_y: u32, mouse_left_button_state: u8, mouse_right_button_state: u8) {
    let mouse_px = Pixel::new(mouse_x, mouse_y);
    let state = InputState {
        time,
        dt,
        frame,
        mouse: MouseState {
            position: mouse_px.to_pos(),
            left_button: mouse_left_button_state.into(),
            right_button: mouse_right_button_state.into(),
        }
    };
    SCENE.as_mut().unwrap().update(state);
}

#[no_mangle]
pub unsafe extern fn render() {
    SCENE.as_mut().unwrap().render(&mut FRAME_BUFFER);
}
