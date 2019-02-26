#[macro_use]
extern crate coord;
extern crate cairo;
extern crate rand;
use cairo::{Context, Format, ImageSurface};
use coord::math::VecFloat;
use std::f64::consts::PI;
use std::fs::File;

mod particle;

fn main() {
    let mut file = File::create("output.png").expect("Couldn’t create file.");

    let n_particles = 50;
    let screen_x = 800;
    let screen_y = 800;

    let mut particles = Vec::new();

    for _ in 0..n_particles {
        let mut p = particle::Particle::init();

        p.reset()
            .set_radius(4.0)
            .set_bounds(screen_x as f32, screen_y as f32)
            .randomize();

        particles.push(p);
    }

    let surface =
        ImageSurface::create(Format::ARgb32, 600, 600).expect("Couldn’t create a surface!");

    let context = Context::new(&surface);

    context.set_source_rgb(1.0, 1.0, 1.0);
    context.fill();

    context.set_source_rgb(0.0, 0.0, 0.0);

    for particle in &particles {
        let x = particle.position.x;
        let y = particle.position.y;
        let r = particle.radius;

        context.arc(x as f64, y as f64, r as f64, 0.0, PI * 2.0);

        context.fill();
    }

    draw_lines(particles, context);

    surface
        .write_to_png(&mut file)
        .expect("Couldnt not write to png");
}

fn draw_lines(particles: Vec<particle::Particle>, context: Context) {
    let n_particles = particles.len();
    let cutover_distance = 150.0;

    context.set_source_rgb(0.0, 0.0, 0.0);

    for i in 0..n_particles {
        for j in (i + 1)..n_particles {
            let p1 = &particles[i];
            let p2 = &particles[j];

            if (p1.position - p2.position).length() < cutover_distance {
                context.move_to(p1.position.x as f64, p1.position.y as f64);
                context.line_to(p2.position.x as f64, p2.position.y as f64);
                context.stroke();
            }
        }
    }
}
