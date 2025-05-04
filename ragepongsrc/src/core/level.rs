use godot::{builtin::{Array, Vector2}, classes::{INode2D, Node2D}, obj::{Base, Gd}, prelude::{godot_api, GodotClass}};

use crate::obstacles::laser_gate::LaserGate;

use super::level_end::LevelEnd;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Level {
    #[export]
    obstacles: Array<Gd<Node2D>>,
    #[export]
    level_end: Option<Gd<LevelEnd>>,
    #[export]
    player_start: Option<Gd<Node2D>>,
    #[export]
    pong_start: Option<Gd<Node2D>>,
    #[export]
    pong_direction: Vector2,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Level {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            obstacles: Default::default(),
            level_end: None,
            player_start: None,
            pong_start: None,
            pong_direction: Vector2::new(0.0, 0.0),
            base,
        }
    }
}

#[godot_api]
impl Level {
    #[func]
    pub fn get_player_start_position(&self) -> Vector2 {
        let player_start = match &self.player_start {
            None => Vector2::new(0.0, 0.0),
            Some(start) => start.get_position()
        };

        return player_start;
    }

    #[func]
    pub fn get_pong_start_position(&self) -> Vector2 {
        let pong_start = match &self.pong_start {
            None => Vector2::new(0.0, 0.0),
            Some(start) => start.get_position()
        };

        return pong_start;
    }

    #[func]
    pub fn reset_obstacles(&self) {
        for o in self.obstacles.iter_shared() {
            if o.get_class() == "LaserGate".into() {
                let mut gate = o.cast::<LaserGate>();
                gate.bind_mut().reset();
            }
        }
    }

}
