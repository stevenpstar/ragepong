use core::panic;

use godot::{builtin::Vector2, classes::{ Area2D, CharacterBody2D, ICharacterBody2D, Input, Node2D }, global::{godot_print, move_toward}, obj::{Base, Gd, WithBaseField}, prelude::{godot_api, GodotClass}};

use crate::{core::gamestate::GameState, player::pong::Pong};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f64,
    #[export]
    jump_velocity: f64,
    jump_count: f32,
    jump_counter: f32,
    #[export]
    gamestate: Option<Gd<GameState>>,
    #[export]
    hittingbox: Option<Gd<Area2D>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {

    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Player Loaded (from rust!)");

        Self {
            speed: 300.0,
            jump_velocity: -200.0,
            jump_count: 4.0,
            jump_counter: 0.0, 
            gamestate: None,
            hittingbox: None,
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("This should be called on ready");
        let hb: &mut Gd<Area2D> = match &mut self.hittingbox {
            None => panic!("Hitting box not defined"),
            Some(hb) => hb
        };

        hb.signals().body_entered().connect(move |body| Self::on_body_entered(body));
        hb.signals().body_exited().connect(move |body| Self::on_body_exited(body));

        godot_print!("signal list: ");
    }

    fn physics_process(&mut self, delta: f64) {
        
        let input = Input::singleton();
        let g_speed = match &self.gamestate {
            None => panic!("No gamestate!"),
            Some(gs) => gs.bind().get_gamespeed(),
        };

        self.jump(&input);

        if !self.base().is_on_floor() {
            let mut new_velocity = self.base().get_velocity();
            new_velocity.x += self.base().get_gravity().x * delta as f32;
            new_velocity.y += self.base().get_gravity().y * delta as f32;
            self.base_mut().set_velocity(new_velocity);
        } else {
            // Reset jump counter whenever we hit the floor
            self.jump_counter = 0.0;
        }

        let direction = input.get_axis("move_left", "move_right");
        if direction != 0.0 {
            let vel_x = direction * self.speed as f32 * g_speed as f32;
            let vel_y = self.base().get_velocity().y as f32;
            self.base_mut().set_velocity(Vector2::new(vel_x, vel_y));
        } else {
            let vel_x = move_toward(self.base().get_velocity().x as f64, 0.0, self.speed * g_speed);
            let vel_y = self.base().get_velocity().y as f32;
            self.base_mut().set_velocity(Vector2::new(vel_x as f32, vel_y));
        }

        {
            let mut new_vel = self.base().get_velocity();
            new_vel.y *= g_speed as f32;
            self.base_mut().set_velocity(new_vel);
        }

        self.base_mut().move_and_slide();

        if input.is_action_just_released("shoot") {
            let h_box = match &self.hittingbox {
                None => panic!("We should have a hitting box"),
                Some(hb) => hb,
            };
            let balls = h_box.get_overlapping_bodies();
            for ball in balls.iter_shared() {
                if ball.get_class() == "Pong".into() {
                    let mut b = ball.cast::<Pong>();
                    b.bind_mut().reverse_direction();
                }
            }
        }
    }
}

impl Player {

    fn on_body_entered(body: Gd<Node2D>) {
        if body.get_class() == "Pong".into() {
            godot_print!("ball hit {}", body.get_class());
        }
    }

    fn on_body_exited(body: Gd<Node2D>) {
        godot_print!("ball left {}", body);
    }

    fn can_jump(&mut self) -> bool {
        if self.base().is_on_floor() {
            return true;
        }
        if self.jump_counter < self.jump_count {
            return true;
        }
        return false;
    }

    fn tick_jump(&mut self) {
        let g_speed = match &self.gamestate {
            None => panic!("No gamestate!"),
            Some(gs) => gs.bind().get_gamespeed() as f32,
        };
        if self.jump_counter < self.jump_count {
            self.jump_counter += 1.0 * g_speed;
        }  
    }

    fn jump(&mut self, input: &Gd<Input>) {

        let _game_speed = match &self.gamestate {
            None => panic!("No gamestate!"),
            Some(gs) => gs.bind().get_gamespeed() as f32,
        };

       if input.is_action_just_pressed("jump") && self.base().is_on_floor() {
           // zero out velocity for new jump
           let mut new_vel = self.base().get_velocity();
           new_vel.y = 0.0;
           self.base_mut().set_velocity(new_vel);
       }
       if input.is_action_pressed("jump") && self.can_jump() {
           godot_print!("Jumping!");
           let mut new_vel = self.base().get_velocity();
           new_vel.y += self.jump_velocity as f32;
           self.base_mut().set_velocity(new_vel);
           self.tick_jump();
       }

    }
}
