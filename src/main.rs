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

    let target_color = [1.0, 0.0, 0.0, 1.0];

    for e in window {
        e.draw_2d(|c, g| {
           clear(target_color, g);

           let original = Ellipse::new([1.0, 1.0, 1.0, 1.0]);
           original.draw([50.0, 50.0, 500.0, 500.0], &c.draw_state, c.transform, g);

           let original = Ellipse::new([1.0, 0.0, 0.0, 0.5]);
           original.draw([300.0, 300.0, 100.0, 100.0], &c.draw_state, c.transform, g);
        });
    }
}
