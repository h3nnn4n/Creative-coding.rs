#[macro_use]
extern crate coord;
extern crate cairo;
extern crate rand;
use coord::math::VecFloat;
use random_color::{Color, Luminosity, RandomColor};

mod color_manager;
mod context_manager;
mod particle;

fn main() {
    let n_particles = 50;
    let n_colors = 6;
    let n_moves = 75;
    let move_range = 8.0;
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

    context
        .set_source_rgba(1.0, 1.0, 1.0, 1.0)
        .paint()
        .set_line_width(1.0);

    context
        .circle((screen_x / 2) as f32, (screen_y / 2) as f32, 250.0)
        .clip();

    let (r, g, b) = color_manager::rgb_array_to_tuple(
        RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Dark)
            .to_rgb_array(),
    );

    context.set_source_rgba(r, g, b, 0.075);

    draw_plexus(n_moves, move_range, &mut particles, &mut context);

    let (r, g, b) = color_manager::rgb_array_to_tuple(
        RandomColor::new()
            .hue(Color::Red)
            .luminosity(Luminosity::Dark)
            .to_rgb_array(),
    );

    context.set_source_rgba(r, g, b, 0.075);

    draw_plexus(n_moves, move_range, &mut particles, &mut context);

    context.save();
}

fn draw_plexus(
    n_moves: i32,
    move_range: f32,
    particles: &mut Vec<particle::Particle>,
    context: &mut context_manager::ContextManager,
) {
    for _ in 0..n_moves {
        for particle in particles.iter_mut() {
            particle.random_move(move_range);
        }

        draw_lines(&particles, context);
    }
}

fn draw_lines(particles: &Vec<particle::Particle>, context: &mut context_manager::ContextManager) {
    let n_particles = particles.len();
    let cutover_distance = 150.0;

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
