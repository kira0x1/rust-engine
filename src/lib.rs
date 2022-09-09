extern "C" {
    fn js_clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32);
}

pub fn clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        js_clear_screen_to_color(red, green, blue, alpha);
    }
}
