use crate::{Renderable, WIDTH, HEIGHT, InputState, ButtonState, Vec2, Color};
use crate::rng::Rng;

struct Particle {
    color: Color,
    start_frame: u32,
    lifetime: u32,
    pos: Vec2,
    velocity: Vec2,
}

impl Particle {
    fn new(pos: Vec2, velocity: Vec2, start_frame: u32, lifetime: u32) -> Particle {
        Particle {
            color: Color::new(0.85, 0.32, 0.13, 1.0),
            start_frame,
            lifetime,
            velocity,
            pos,
        }
    }
}

pub struct Scene {
    state: InputState,
    particles: Vec<Particle>,
    rng: Rng,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            state: InputState::default(),
            particles: Vec::new(),
            rng: Rng::new(1312),
        }
    }
}

impl Renderable for Scene {
    fn render(&self, buffer: &mut [u32; WIDTH * HEIGHT]) {
        let black = Color::new(0.0, 0.0, 0.0, 1.0).to_u32();
        buffer.fill(black);
        for p in &self.particles {
            if let Some(px) = p.pos.to_px() {
                let i = px.to_index();
                buffer[i] = p.color.to_u32();
            }
        }
    }

    fn update(&mut self, state: InputState) {
        self.state = state;
        if matches!(self.state.mouse.left_button, ButtonState::Pressed) {
            for _ in 0..10 {
                let init_velocity = Vec2::new(self.rng.gen_f32_range(-1.0, 1.0), self.rng.gen_f32_range(3.0, 4.0));
                self.particles.push(Particle::new(self.state.mouse.position, init_velocity, state.frame, 100));
            }
        }
        self.particles.retain(|p| p.start_frame + p.lifetime > state.frame);
        let gravity = Vec2::new(0.0, -0.2);
        if state.dt > 0.0 {
            for p in &mut self.particles {
                p.pos = p.pos + p.velocity;
                p.velocity = p.velocity + gravity;
            }
        }
    }
}