use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::global::{Key, MouseButton};
use godot::obj::WithBaseField;
use godot::prelude::*;
use std::f64::consts::PI;

use crate::bullet::Bullet;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,

    #[export]
    speed: f64,
    #[export]
    initial_front: f64,
    #[export]
    bullet_speed: f32,

    bullet: Gd<PackedScene>,
    counter: i32,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            speed: 1.,
            initial_front: 0.,
            bullet: PackedScene::new_gd(),
            bullet_speed: 0.,
            counter: 0,
        }
    }

    fn ready(&mut self) {
        self.bullet = load("scenes/bullet.tscn");
        // self.base_mut().set_gravity_scale(0.);
    }

    fn process(&mut self, delta: f64) {
        self.movement_handler(delta);
        self.rotation_handler();
        self.attack_handler();
    }
}

impl Player {
    fn movement_handler(&mut self, delta: f64) {
        let input = Input::singleton();
        let four_directions = (
            input.is_key_pressed(Key::W),
            input.is_key_pressed(Key::A),
            input.is_key_pressed(Key::S),
            input.is_key_pressed(Key::D),
        );
        #[rustfmt::skip]
        let move_direction = match four_directions {
            (true , false, false, false) => Some(0f64),
            (true , false, false, true ) => Some(PI / 4.),
            (false, false, false, true ) => Some(PI / 2.),
            (false, false, true , true ) => Some(PI - PI / 4.),
            (false, false, true , false) => Some(PI),
            (false, true , true , false) => Some(PI + PI / 4.),
            (false, true , false, false) => Some(PI + PI / 2.),
            (true , true , false, false) => Some(-PI / 4.),
            _ => None,
        };
        let velocity = if let Some(direction) = move_direction {
            Vector2::new(
                ((direction + self.initial_front.to_radians()).cos() * self.speed * delta) as f32,
                ((direction + self.initial_front.to_radians()).sin() * self.speed * delta) as f32,
            )
        } else {
            Vector2::ZERO
        };
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
        self.counter += 1;
        if self.counter % 30 == 0 {
            // println!("\ndirection_vec2: {}\n", velocity);
            // println!();
            // println!("four_directions: {:?}", four_directions);
            // println!("velocity: {}", velocity);
            // println!("{:?}", viewport);
            // println!("mouse_position: {}", mouse_position);
            // println!("self_position: {}", self_position);
            // println!("angle_to_mouse: {}", angle_to_mouse);
            // println!("self_rotation_before: {}", self_rotation_before);
            // println!("mouse_position: {}", mouse_position);
            // println!("face_to: {}", face_to);
            // println!("self_rotation: {}", self_rotation);
        }
    }

    fn rotation_handler(&mut self) {
        let viewport = self.base().get_viewport().unwrap();
        let camera = viewport.get_camera_2d().unwrap();
        let camera_position = camera.get_position();
        let camera_scale = camera.get_scale();
        let windows_size = self.base().get_window().unwrap().get_size();
        // YOOO FIRST TRY LMAOOOOOOO
        let mouse_position = viewport.get_mouse_position()
            + (camera_position
                - (Vector2::new(
                    windows_size.x as f32 / camera_scale.x,
                    windows_size.y as f32 / camera_scale.y,
                ) / 2.));
        let self_position = self.base().get_global_position();
        let angle_to_mouse = (self_position.y - mouse_position.y)
            .atan2(self_position.x - mouse_position.x)
            + self.initial_front.to_radians() as f32;
        // let face_to = self.base().get_global_position().angle_to_point(mouse_position);
        // let self_rotation_before = self.base().get_global_rotation();
        self.base_mut().set_rotation(angle_to_mouse);
        // let self_rotation = self.base().get_rotation();
    }

    const DISTANCE_BULLET: Vector2 = Vector2::new(0f32, -24f32);

    fn attack_handler(&mut self) {
        let input = Input::singleton();
        if input.is_mouse_button_pressed(MouseButton::LEFT) {
            println!("shoot");
            let mut bullet = self.bullet.instantiate_as::<Bullet>();
            bullet.set_position(
                self.base().get_position()
                    + Self::DISTANCE_BULLET.rotated(self.base().get_rotation()),
            );
            bullet.set_linear_velocity(Vector2::new(
                (self.base().get_rotation() + self.initial_front.to_radians() as f32).cos() * self.bullet_speed,
                (self.base().get_rotation() + self.initial_front.to_radians() as f32).sin() * self.bullet_speed
            ));
            self.base_mut().get_parent().unwrap().add_child(bullet.clone().upcast());
        }
    }
}
