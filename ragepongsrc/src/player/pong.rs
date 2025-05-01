use godot::{builtin::Vector2, classes::{CharacterBody2D, ICharacterBody2D}, obj::{Base, Gd, WithBaseField}, prelude::{godot_api, GodotClass}};

use crate::core::gamestate::GameState;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Pong {
    #[export]
    speed: f64,
    vel_x: f32,
    vel_y: f32,
    #[export]
    gamestate: Option<Gd<GameState>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Pong {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 20.0,
            vel_x: 1.0,
            vel_y: 1.0,
            gamestate: None,
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        {
            let g_speed = match &self.gamestate {
                None => 1.0,
                Some(gs) => gs.bind().get_gamespeed() as f32,
            };
            let vel_x: f32 = self.vel_x * g_speed * self.speed as f32;
            let vel_y: f32 = self.vel_y * g_speed * self.speed as f32;
            self.base_mut().set_velocity(Vector2::new(vel_x, vel_y));
        }
        self.base_mut().move_and_slide();

        {
            let collision = self.base_mut().get_last_slide_collision();
            let collided: bool = match collision {
                None => false,
                Some(_) => true,
            };
            if collided {
                self.reverse_direction();
            }
        }
    }
}

#[godot_api]
impl Pong {
    #[func]
    pub fn reverse_direction(&mut self) {
        // TODO: Collision / bounce calculation will need to get math-y later when things get more
        // complicated
        if self.base().is_on_floor() || self.base().is_on_ceiling() {
            self.vel_y = -self.vel_y;
        } else {
            self.vel_x = -self.vel_x;
        }
    }

    #[func]
    pub fn hit_direction(&mut self, dir: Vector2) {
        self.vel_x = dir.x;
        self.vel_y = dir.y;
    }
}
