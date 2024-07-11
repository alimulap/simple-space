use godot::prelude::*;

mod bullet;
mod camera2d;
mod player;

struct SimpleSpace;

#[gdextension]
unsafe impl ExtensionLibrary for SimpleSpace {}
