use crate::{Renderable, WIDTH, HEIGHT, InputState, ButtonState};

struct Particle {
    color: (f32, f32, f32, f32),
    x: f32,
    y: f32,
}

pub struct Scene {
    state: InputState,
    particles: Vec<Particle>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            state: InputState::default(),
            particles: Vec::new(),
        }
    }
}

impl Renderable for Scene {
    fn render(&self, buffer: &mut [u32; WIDTH * HEIGHT]) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.state.mouse.left_button {
                    ButtonState::Pressed => buffer[y * WIDTH + x] = (x ^ y) as u32 - self.state.time as u32 | 0xFF_00_00_00,
                    ButtonState::Held => todo!(),
                    ButtonState::Released => buffer[y * WIDTH + x] = (x ^ y) as u32 + self.state.time as u32 | 0xFF_00_00_00,
                }
            }
        }
    }

    fn update(&mut self, state: InputState) {
        self.state = state;
    }
}