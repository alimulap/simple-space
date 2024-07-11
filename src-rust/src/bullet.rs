use godot::{
    engine::{IRigidBody2D, RigidBody2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Bullet {
    base: Base<RigidBody2D>,
}

#[godot_api]
impl IRigidBody2D for Bullet {
    fn init(base: Base<RigidBody2D>) -> Self {
        Self { base }
    }

    fn process(&mut self, _delta: f64) {}
}

impl Bullet {}
