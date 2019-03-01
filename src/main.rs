#[macro_use]
extern crate coord;
extern crate cairo;
extern crate rand;
use coord::math::VecFloat;

mod context_manager;
mod particle;

fn main() {
    let n_particles = 50;
    let screen_x = 800;
    let screen_y = 800;

    let mut particles = Vec::new();

    let mut context = context_manager::ContextManager::init(screen_x, screen_y);
    context.set_filename("output.png".to_string());

    for _ in 0..n_particles {
        let mut p = particle::Particle::init();

        p.reset()
            .set_radius(4.0)
            .set_bounds(screen_x as f32, screen_y as f32)
            .randomize();

        particles.push(p);
    }

    context.set_source_rgb(1.0, 1.0, 1.0);
    context.fill();

    context.set_source_rgb(0.0, 0.0, 0.0);

    for particle in &particles {
        let x = particle.position.x;
        let y = particle.position.y;
        let r = particle.radius;

        context.circle(x, y, r);

        context.fill();
    }

    draw_lines(particles, &mut context);

    context.save();
}

fn draw_lines(particles: Vec<particle::Particle>, context: &mut context_manager::ContextManager) {
    let n_particles = particles.len();
    let cutover_distance = 150.0;

    context.set_source_rgb(0.0, 0.0, 0.0);

    for i in 0..n_particles {
        for j in (i + 1)..n_particles {
            let p1 = &particles[i];
            let p2 = &particles[j];

            if (p1.position - p2.position).length() < cutover_distance {
                context
                    .move_to(p1.position.x, p1.position.y)
                    .line_to(p2.position.x, p2.position.y)
                    .stroke();
            }
        }
    }
}
