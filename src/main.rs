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
    let mut mouse_position = (0.0, 0.0);

    for e in window {
        e.draw_2d(|c, g| {
           clear(target_color, g);

           let playing_surface = Rectangle::new([1.0, 1.0, 1.0, 1.0]);
           playing_surface.draw([50.0, 50.0, 500.0, 500.0], &c.draw_state, c.transform, g);

           let candidate1 = Rectangle::new(target_color);
           candidate1.draw([300.0, 300.0, 100.0, 100.0], &c.draw_state, c.transform, g);

           let candidate2 = Rectangle::new([0.0, 1.0, 0.0, 1.0]);
           candidate2.draw([100.0, 200.0, 100.0, 100.0], &c.draw_state, c.transform, g);

           let candidate3 = Rectangle::new([0.0, 0.0, 1.0, 1.0]);
           candidate3.draw([300.0, 100.0, 100.0, 100.0], &c.draw_state, c.transform, g);
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
