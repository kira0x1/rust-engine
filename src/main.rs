mod GameObject;

use raylib::prelude::*;
use GameObject::create_template_object;

fn main() {
    let mut go = create_template_object();

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("meow")
        .build();

    let mut counter = 0.0;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Meow!", 12, 12, 20, Color::BLACK);

        d.draw_poly(go.pos, 4, go.size, 0.0, Color::BLACK);

        let ftime = d.get_frame_time();
        counter += ftime;

        go.pos.x += 25.0 * d.get_frame_time();

        if (counter > 1.0) {
            if go.size == 16.0 {
                go.size = 32.0;
            } else {
                go.size = 16.0;
            }

            counter = 0.0;
        }
    }
}