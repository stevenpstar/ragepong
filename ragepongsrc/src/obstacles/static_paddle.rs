use godot::{builtin::Vector2, classes::{ CollisionShape2D, IAnimatableBody2D, Node2D, AnimatableBody2D}, global::godot_print, obj::{Base, Gd, OnReady, WithBaseField}, prelude::{godot_api, GodotClass}};

use crate::player::pong::Pong;

#[derive(GodotClass)]
#[class(base=AnimatableBody2D)]
pub struct StaticPaddle {
    #[export]
    speed: f32,
    #[export]
    vertical: bool,
    #[export]
    min_point: Option<Gd<Node2D>>,
    #[export]
    max_point: Option<Gd<Node2D>>,
    #[export]
    bounds: Option<Gd<CollisionShape2D>>,
    pong: OnReady<Gd<Pong>>,
    base: Base<AnimatableBody2D>,
}

#[godot_api]
impl IAnimatableBody2D for StaticPaddle {
    fn init(base: Base<AnimatableBody2D>) -> Self {
        Self {
            speed: 0.5,
            vertical: true,
            min_point: None,
            max_point: None,
            bounds: None,
            pong: OnReady::from_node("/root/Game/Pong"),
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {

        let min_point = match &self.min_point {
            None => {
                godot_print!("No bounds set for Static Paddle");
                panic!("No bounds set");
            },
            Some(b) => b.get_global_position()
        };

        let max_point = match &self.max_point {
            None => {
                godot_print!("No bounds set for Static Paddle");
                panic!("No bounds set");
            },
            Some(b) => b.get_global_position()
        };

        let bounds = match & self.bounds {
            None => {
                 godot_print!("No bounds set for Static Paddle");
                panic!("No bounds set");
            },
            Some(b) => b.get_shape().unwrap().get_rect()
        };


        let mut target_position_x = self.base().get_global_position().x;
        let mut target_position_y = self.base().get_global_position().y;
        if self.vertical {
            target_position_y = self.pong.get_position().y;

            if target_position_y - bounds.size.y / 2.0 < min_point.y {
                target_position_y = min_point.y + bounds.size.y / 2.0;
            } else if target_position_y + bounds.size.y / 2.0 > max_point.y {
                target_position_y = max_point.y - bounds.size.y / 2.0;
            }


        } else {
            target_position_x = self.pong.get_position().x;

            if target_position_x - bounds.size.x / 2.0 < min_point.x {
                target_position_x = min_point.x + bounds.size.x / 2.0;
            } else if target_position_x + bounds.size.x / 2.0 > max_point.x {
                target_position_x = max_point.x - bounds.size.x / 2.0;
            }

        }



       let mut pos = self.base().get_global_position();
       pos = pos.lerp(Vector2::new(target_position_x, target_position_y), self.speed);
       self.base_mut().set_global_position(pos);
    }
}
