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

           let playing_surface = Ellipse::new([1.0, 1.0, 1.0, 1.0]);
           playing_surface.draw([50.0, 50.0, 500.0, 500.0], &c.draw_state, c.transform, g);

           let candidate1 = Ellipse::new(target_color);
           candidate1.draw([300.0, 300.0, 100.0, 100.0], &c.draw_state, c.transform, g);

           let candidate2 = Ellipse::new([0.0, 1.0, 0.0, 1.0]);
           candidate2.draw([100.0, 200.0, 100.0, 100.0], &c.draw_state, c.transform, g);

           let candidate3 = Ellipse::new([0.0, 0.0, 1.0, 1.0]);
           candidate3.draw([300.0, 100.0, 100.0, 100.0], &c.draw_state, c.transform, g);
        });
    }
}
