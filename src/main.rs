extern crate piston_window;

use piston_window::*;

struct Candidate {
    coordinates: [f64; 4],
    color: [f32; 4],
}

fn main() {
    let window: PistonWindow = WindowSettings::new(
            "colors",
            [600, 600]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let target_color = [1.0, 0.0, 0.0, 1.0];
    let mut mouse_position = (0.0, 0.0);
    let candidates = vec![
        Candidate { coordinates: [300.0, 300.0, 100.0, 100.0], color: target_color },
        Candidate { coordinates: [100.0, 200.0, 100.0, 100.0], color: [0.0, 1.0, 0.0, 1.0] },
        Candidate { coordinates: [300.0, 100.0, 100.0, 100.0], color: [0.0, 0.0, 1.0, 1.0] },
    ];

    for e in window {
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
                println!("press {:?}", mouse_position);
            }
        };

    }
}
