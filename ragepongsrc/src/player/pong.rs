use godot::{builtin::Vector2, classes::{Area2D, CharacterBody2D, ICharacterBody2D, Node2D, Sprite2D}, global::godot_print, obj::{Base, Gd, OnReady, WithBaseField}, prelude::{godot_api, GodotClass}};

use crate::{core::{colour_component::ColourComponent, colours::Colour}, obstacles::{laser_gate::LaserGate, pong_lock::PongLock}};

use super::player::Player;

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
    white_sprite: OnReady<Gd<Sprite2D>>,
    red_sprite: OnReady<Gd<Sprite2D>>,
    blue_sprite: OnReady<Gd<Sprite2D>>,
    green_sprite: OnReady<Gd<Sprite2D>>,
    colour: OnReady<Gd<ColourComponent>>,
    locked: bool,
    locked_position: Vector2,
    vel_x: f32,
    vel_y: f32,
    game_speed: f32,
    level_finished: bool,
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
            colour: OnReady::from_node("ColourComponent"),
            white_sprite: OnReady::from_node("WhitePong"),
            red_sprite: OnReady::from_node("RedPong"),
            blue_sprite: OnReady::from_node("BluePong"),
            green_sprite: OnReady::from_node("GreenPong"),
            locked: false,
            locked_position: Vector2::new(0.0, 0.0),
            vel_x: 1.0,
            vel_y: 1.0,
            game_speed: 1.0,
            level_finished: false,
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

        hurt.signals()
            .area_entered()
            .connect_obj(&this, |s: &mut Self, area| {
                s.on_area_entered(area);
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

                    let col_obj = match collision {
                        None => panic!("Collision vanished!"),
                        Some(obj) => obj
                    };

                    let collider = match col_obj.get_collider() {
                        None => panic!("No collider"),
                        Some(col) => col
                    };

                    if collider.get_class() != "Player".into() {
                        self.reverse_direction();
                    }
                }
            }

        } else {
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
        self.level_finished = false;
    }

    #[func]
    pub fn lock(&mut self, lock: Gd<PongLock>) {
        if self.level_finished {
            return;
        }
        self.locked = true;
        self.locked_position = lock.get_position();
    }

    #[func]
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    #[func]
    pub fn get_colour(&mut self) -> Colour {
        return self.colour.bind().get_obj_colour();
    }

    #[func]
    pub fn get_pong_speed(&self) -> f64 {
        return self.speed;
    }

    #[func]
    pub fn set_level_finished(&mut self, fin: bool) {
        self.level_finished = fin;
    }

    #[func]
    pub fn get_level_fin(&self) -> bool {
        return self.level_finished;
    }

    #[func]
    pub fn is_locked(&self) -> bool {
        return self.locked;
    }
    
    pub fn set_pong_colour(&mut self, col: &Colour) {

        let colour_comp = &mut self.colour;

        colour_comp.bind_mut().set_obj_colour(col);

        self.white_sprite.set_visible(false);
        self.red_sprite.set_visible(false);
        self.blue_sprite.set_visible(false);
        self.green_sprite.set_visible(false);
        match self.get_colour() {
            Colour::White => self.white_sprite.set_visible(true),
            Colour::Red => self.red_sprite.set_visible(true),
            Colour::Blue => self.blue_sprite.set_visible(true),
            Colour::Green => self.green_sprite.set_visible(true),
        };
    }

    pub fn update_game_speed(&mut self, speed: f32) {
        self.game_speed = speed;
    }

    fn on_hazard_entered(&mut self, _body: Gd<Node2D>) {
        self.reset();
    }

    fn on_area_entered(&mut self, area: Gd<Area2D>) {
        let parent = match area.get_parent() {
            None => return, // ignore
            Some(p) => p
        };


        godot_print!("parent area entered {}", parent.get_class());

        if parent.get_class() == "Player".into() {
            let mut player = parent.cast::<Player>();
            let colour = self.get_colour();
            if colour == Colour::Red {
                player.bind_mut().kill();
            }
        } else if area.get_class() == "LaserGate".into() {
            godot_print!("Hit laser gate!");
            let mut gate = area.cast::<LaserGate>();
            let ball_colour = self.get_colour();
            let laser_gate_colour = gate.bind_mut().get_colour();
            if ball_colour != laser_gate_colour && gate.bind().get_is_open() {
                self.reset();
                godot_print!("should reset");
            } else {
                godot_print!("should not reset");
            }
        }

    }

}
