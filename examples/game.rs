use rust_engine::Key;

fn main() {
    rust_engine::set_event_handler(move |key| match key {
        Key::Left => rust_engine::clear_screen_to_color(1.0, 0.0, 0.0, 1.0),
        Key::Right => rust_engine::clear_screen_to_color(0.0, 1.0, 0.0, 1.0),
        Key::Up => rust_engine::clear_screen_to_color(0.0, 0.0, 1.0, 1.0),
        Key::Down => rust_engine::clear_screen_to_color(0.0, 1.0, 1.0, 1.0),
        Key::Space => rust_engine::clear_screen_to_color(1.0, 1.0, 0.0, 1.0),
    })
}
