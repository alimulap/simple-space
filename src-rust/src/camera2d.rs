use godot::{obj::WithBaseField, prelude::*};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Camera2D)]
struct Camera2DCustom {
    player: Option<Gd<Player>>,
    base: Base<Camera2D>,
    counter: i32,
}

#[godot_api]
impl ICamera2D for Camera2DCustom {
    fn init(base: Base<Camera2D>) -> Self {
        Self {
            player: None,
            base,
            counter: 0,
        }
    }

    fn ready(&mut self) {
        println!(
            "{}",
            self.base()
                .get_tree()
                .unwrap()
                .get_nodes_in_group("player".into())
                .len()
        );
        self.player = Some(
            self.base()
                .get_tree()
                .unwrap()
                .get_nodes_in_group("player".into())
                .get(0)
                .unwrap()
                .cast::<Player>(),
        );
    }

    fn process(&mut self, _delta: f64) {
        let player_position = self.player.as_ref().unwrap().get_position();
        let self_position = self.base().get_position();
        let next_camera_position = self_position.lerp(player_position, 0.1);
        self.base_mut().set_position(next_camera_position);

        self.counter += 1;
        if self.counter % 30 == 0 {
            // println!("camera_position: {}", self_position);
        }
    }
}
