use godot::{builtin::Vector2, classes::{Area2D, CharacterBody2D, ICharacterBody2D, Node2D}, obj::{Base, Gd, WithBaseField}, prelude::{godot_api, GodotClass}};

use crate::obstacles::pong_lock::PongLock;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Pong {
    #[export]
    speed: f64,
    #[export]
    start_point: Option<Gd<Node2D>>,
    #[export]
    start_dir: Vector2,
    #[export]
    hurtbox: Option<Gd<Area2D>>,
    locked: bool,
    locked_position: Vector2,
    vel_x: f32,
    vel_y: f32,
    game_speed: f32,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Pong {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 20.0,
            start_point: None,
            start_dir: Vector2::new(0.0, 0.0),
            hurtbox: None,
            locked: false,
            locked_position: Vector2::new(0.0, 0.0),
            vel_x: 1.0,
            vel_y: 1.0,
            game_speed: 1.0,
            base,
        }
    }

    fn ready(&mut self) {
        let this = self.to_gd();

        let hurt: &mut Gd<Area2D> = match &mut self.hurtbox {
            None => panic!("Hurt box not defined"),
            Some(hb) => hb
        };

        hurt.signals()
            .body_entered()
            .connect_obj(&this, |s: &mut Self, body| {
                s.on_hazard_entered(body);
        });

        self.reset();
    }

    fn physics_process(&mut self, _delta: f64) {
        if !self.locked {
            {
                let vel_x: f32 = self.vel_x * self.game_speed * self.speed as f32;
                let vel_y: f32 = self.vel_y * self.game_speed * self.speed as f32;
                self.base_mut().set_velocity(Vector2::new(vel_x, vel_y));
            }
            self.base_mut().move_and_slide();
            {
                let collision = self.base_mut().get_last_slide_collision();
                let collided: bool = match collision {
                    None => false,
                    Some(_) => {
                        true
                    },
                };
                if collided {
                    self.reverse_direction();
                }
            }

        } else {
            //let vel_x = move_toward(self.base().get_position().x as f64,
            //    self.locked_position.x as f64, 0.2);
            //let vel_y = move_toward(self.base().get_position().y as f64,
            //    self.locked_position.y as f64, 0.2);
            //self.base_mut().set_velocity(Vector2::new(vel_x as f32, vel_y as f32));
            //self.base_mut().move_and_slide();
            let l_pos = self.locked_position;
            self.base_mut().set_position(l_pos);
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

    pub fn reset(&mut self) {
        let position = match &self.start_point {
            None => Vector2::new(0.0, 0.0),
            Some(sp) => sp.get_position()
        };
        self.base_mut().set_position(position);
        self.vel_x = self.start_dir.x;
        self.vel_y = self.start_dir.y;
        self.locked = false;
    }

    #[func]
    pub fn lock(&mut self, lock: Gd<PongLock>) {
        self.locked = true;
        self.locked_position = lock.get_position();
    }

    #[func]
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn update_game_speed(&mut self, speed: f32) {
        self.game_speed = speed;
    }

    fn on_hazard_entered(&mut self, _body: Gd<Node2D>) {
        self.reset();
    }
}
