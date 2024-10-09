use raylib::prelude::*;

pub struct GameObject {
    pub name: String,
    pub pos: Vector2,
    pub speed: f32,
    pub size: f32,
}

pub fn create_template_object() -> GameObject {
    GameObject {
        name: String::from("GameObject"),
        pos: Vector2::new(640.0 / 2.0, 480.0 / 2.0),
        speed: 25.0,
        size: 16.0,
    }
}