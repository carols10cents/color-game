extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

#[derive(Debug,Copy,Clone,PartialEq)]
struct Candidate {
    coordinates: [f64; 4],
    color: [f32; 4],
}

fn generate_candidates(difficulty: f32) -> Vec<Candidate> {
    let mut candidate_positions = vec![
        (100.0, 100.0), (100.0, 250.0), (100.0, 400.0),
        (250.0, 100.0), (250.0, 250.0), (250.0, 400.0),
        (400.0, 100.0), (400.0, 250.0), (400.0, 400.0),
    ];
    let num_candidates = 5;
    let mut candidates = vec![];
    let random_component = rand::thread_rng().gen_range(450, 900) as f32 / 1000.0;
    let random_index = rand::thread_rng().gen_range(0, 3);
    let mut random_color = [
        0.3,
        0.3,
        0.3,
        1.0,
    ];
    random_color[random_index] = random_component;
    let variance = ::std::f32::consts::PI / difficulty;
    for i in 0..num_candidates {
        let which_position = rand::thread_rng().gen_range(0, candidate_positions.len());
        let pos = candidate_positions.remove(which_position);
        candidates.push(Candidate {
            coordinates: [pos.0, pos.1, 100.0, 100.0],
            color: math::hsv(random_color, variance * i as f32, 1.0, 1.0),
        });
    }
    candidates
}

fn main() {
    let window: PistonWindow = WindowSettings::new(
            "colors",
            [600, 600]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut mouse_position = (0.0, 0.0);
    let mut difficulty = 3.0;
    let mut candidates = generate_candidates(difficulty);

    for e in window {
        if candidates.is_empty() {
            difficulty += 1.0;
            candidates = generate_candidates(difficulty);
        }
        let target_color = candidates[0].color;
        e.draw_2d(|c, g| {
           clear(target_color, g);

           let playing_surface = Rectangle::new([1.0, 1.0, 1.0, 1.0]);
           playing_surface.draw([50.0, 50.0, 500.0, 500.0], &c.draw_state, c.transform, g);

           for candidate in &candidates {
               let r = Rectangle::new(candidate.color);
               r.draw(candidate.coordinates, &c.draw_state, c.transform, g);
           }
        });
        e.mouse_cursor(|x, y| {
            mouse_position = (x, y);
        });
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                if let Some(clicked_on) = candidates.iter().filter(|candidate| {
                    mouse_position.0 > candidate.coordinates[0] &&
                       mouse_position.0 < (candidate.coordinates[0] + candidate.coordinates[2]) &&
                       mouse_position.1 > candidate.coordinates[1] &&
                       mouse_position.1 < (candidate.coordinates[1] + candidate.coordinates[3])
                }).next().cloned() {
                    if clicked_on.color == target_color {
                        candidates.retain(|&r| r != clicked_on);
                    } else {
                        break;
                    }
                }
            }
        };

    }
}
