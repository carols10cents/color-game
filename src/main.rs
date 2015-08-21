extern crate piston_window;

use piston_window::*;

fn main() {
    let window: PistonWindow = WindowSettings::new(
            "colors",
            [600, 600]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    for e in window {
        e.draw_2d(|c, g| {
           clear([1.0; 4], g);
           let original = Ellipse::new([1.0, 0.0, 0.0, 0.5]);
           original.draw([0.0, 0.0, 100.0, 100.0], &c.draw_state, c.transform, g);
        });
    }
}
