use coord::prelude::*;

#[derive(Debug)]
pub struct Particle {
    pub position: Vec2f,
    pub velocity: Vec2f,
    pub bounds: Vec2f,
    pub radius: f32,
}

impl Particle {
    pub fn init() -> Particle {
        Particle {
            position: vec2!(0.0, 0.0),
            velocity: vec2!(0.0, 0.0),
            bounds: vec2!(0.0, 0.0),
            radius: 0.0,
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        *self = Particle::init();

        self
    }

    pub fn set_radius(&mut self, r: f32) -> &mut Self {
        self.radius = r;

        self
    }

    pub fn randomize(&mut self) -> &mut Self {
        self.position.x = rand::random::<f32>() * self.bounds.x;
        self.position.y = rand::random::<f32>() * self.bounds.y;

        self
    }

    pub fn set_bounds(&mut self, x: f32, y: f32) -> &mut Self {
        self.bounds.x = x;
        self.bounds.y = y;

        self
    }
}
