use core::panic;

use godot::{builtin::Vector2, classes::{ AnimatedSprite2D, Area2D, CharacterBody2D, ICharacterBody2D, Input, Line2D, Node2D }, global::{godot_print, move_toward}, obj::{Base, Gd, WithBaseField, WithUserSignals}, prelude::{godot_api, GodotClass}};

use crate::player::pong::Pong;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    alive: bool,
    aiming: bool,
    in_range: bool,
    speed: f64,
    game_speed: f32,
    #[export]
    jump_velocity: f64,
    jump_count: f32,
    jump_counter: f32,
    #[export]
    mkb: bool,
    #[export]
    hittingbox: Option<Gd<Area2D>>,
    #[export]
    hurtbox: Option<Gd<Area2D>>,
    #[export]
    sprite: Option<Gd<AnimatedSprite2D>>,
    #[export]
    start_point: Option<Gd<Node2D>>,
    #[export]
    aim_ind: Option<Gd<Line2D>>,
    #[export]
    aim_ind_in_range: Option<Gd<Line2D>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {

    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Player Loaded (from rust!)");

        Self {
            alive: true,
            aiming: false,
            in_range: false,
            speed: 300.0,
            game_speed: 1.0,
            jump_velocity: -200.0,
            jump_count: 4.0,
            jump_counter: 0.0, 
            mkb: true, // defaults to player using mouse/key board for aiming. Changes dynamically.
            hittingbox: None,
            hurtbox: None,
            sprite: None,
            start_point: None,
            aim_ind: None,
            aim_ind_in_range: None,
            base,
        }
    }

    fn ready(&mut self) {
        let this = self.to_gd();


        godot_print!("This should be called on ready");
        let hb: &mut Gd<Area2D> = match &mut self.hittingbox {
            None => {
                godot_print!("Hitting box not defined?");
                panic!("Heiagea");
            },
            Some(hb) => hb
        };


        let hurt: &mut Gd<Area2D> = match &mut self.hurtbox {
            None => panic!("Hurt box not defeind"),
            Some(hb) => hb
        };

        hb.signals().body_entered()
            .connect_obj(&this, |s: &mut Self, body| {
                s.on_body_entered(body);
            });
        hb.signals().body_exited()
            .connect_obj(&this, |s: &mut Self, body| {
                s.on_body_exited(body);
            });


        hurt.signals()
            .body_entered()
            .connect_obj(&this, |s: &mut Self, body| {
                s.on_hazard_entered(body);
            });

        // set player spawn
        self.reset_player();
    }

    fn physics_process(&mut self, delta: f64) {

        if !self.alive {
            return;
        }
        
        let input = Input::singleton();

        let on_floor: bool = self.base().is_on_floor();

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
        let hit_vert = input.get_axis("hit_up", "hit_down");
        let hit_horiz = input.get_axis("hit_left", "hit_right");

        if self.aiming {

            let aim = match &mut self.aim_ind {
                None => {
                    godot_print!("No aim ind");
                    panic!("ahhh");
                },
                Some(aim) => aim
            };

            let aim_in_range = match &mut self.aim_ind_in_range {
                None => {
                    godot_print!("No aim ind (in range)");
                    panic!("ahhh no in range ind");
                },
                Some(aim) => aim
            };

            let mut player_pos: Vector2 = Default::default();
            {
                player_pos.x = aim.get_position().x;
                player_pos.y = aim.get_position().y;
            }

            aim.set_point_position(1, Vector2::new(
                    player_pos.x + hit_horiz * 50.0,
                    player_pos.y + hit_vert * 50.0)
                );

            aim_in_range.set_point_position(1, Vector2::new(
                    player_pos.x + hit_horiz * 50.0,
                    player_pos.y + hit_vert * 50.0)
                );
        }

        if direction != 0.0 {
            let vel_x = direction * self.speed as f32 * self.game_speed as f32;
            let vel_y = self.base().get_velocity().y as f32;
            self.base_mut().set_velocity(Vector2::new(vel_x, vel_y));
            // play run animation
            let char_sprite: &mut Gd<AnimatedSprite2D> = match &mut self.sprite {
                None => panic!("No animated sprite attached to player"),
                Some(anim_sprite) => anim_sprite,
            };
            if on_floor {
                if direction < 0.0 {
                    char_sprite.set_flip_h(true);
                } else if direction > 0.0 {
                    char_sprite.set_flip_h(false);
                }
                char_sprite.set_animation("run");
            } else {

                if direction < 0.0 {
                    char_sprite.set_flip_h(true);
                } else if direction > 0.0 {
                    char_sprite.set_flip_h(false);
                }

                if vel_y < 0.0 {
                    char_sprite.set_animation("jump");
                } else {
                    char_sprite.set_animation("fall");
                }
            }
        } else {
            let vel_x = move_toward(self.base().get_velocity().x as f64, 0.0, self.speed * self.game_speed as f64);
            let vel_y = self.base().get_velocity().y as f32;
            self.base_mut().set_velocity(Vector2::new(vel_x as f32, vel_y));
            let char_sprite: &mut Gd<AnimatedSprite2D> = match &mut self.sprite {
                None => panic!("No animated sprite attached to player"),
                Some(anim_sprite) => anim_sprite,
            };
            if on_floor {
                if direction < 0.0 {
                    char_sprite.set_flip_h(true);
                } else if direction > 0.0 {
                    char_sprite.set_flip_h(false);
                }
                char_sprite.set_animation("idle");
            } else {
                if vel_y < 0.0 {
                    char_sprite.set_animation("jump");
                } else {
                    char_sprite.set_animation("fall");
                }
            }
        }

        {
            let mut new_vel = self.base().get_velocity();
            new_vel.y *= self.game_speed as f32;
            self.base_mut().set_velocity(new_vel);
        }

        self.base_mut().move_and_slide();

        if input.is_action_pressed("shoot") {
            self.aiming = true;
            if self.in_range {
                match &mut self.aim_ind {
                    None => {
                        godot_print!("Define aim indicator for player");
                        panic!("No player aim ind");
                    },
                    Some(aim) => {
                        aim.set_visible(false);
                    }
                };

                match &mut self.aim_ind_in_range {
                    None => {
                        godot_print!("Define aim indicator for player");
                        panic!("No player aim ind");
                    },
                    Some(aim) => {
                        aim.set_visible(true);
                    }

                };
            } else {

                match &mut self.aim_ind {
                None => {
                    godot_print!("Define aim indicator for player");
                    panic!("No player aim ind");
                },
                Some(aim) => {
                    aim.set_visible(true);
                }
                };

                match &mut self.aim_ind_in_range {
                None => {
                    godot_print!("Define aim indicator for player");
                    panic!("No player aim ind");
                },
                Some(aim) => {
                    aim.set_visible(false);
                }
            };

            }
        }
        else if input.is_action_just_released("shoot") {
            self.aiming = false;
            match &mut self.aim_ind {
                None => {
                    godot_print!("Define aim indicator for player");
                    panic!("No player aim ind");
                },
                Some(aim) => aim.set_visible(false)
            };

            match &mut self.aim_ind_in_range {
                None => {
                    godot_print!("Define aim indicator for player");
                    panic!("No player aim ind");
                },
                Some(aim) => aim.set_visible(false)
            };


            let mouse_position = match &self.base_mut().get_viewport() {
                None => panic!("no viewport"),
                Some(viewport) => viewport.get_mouse_position(),
            };

            let hit_direction_x;
            let hit_direction_y;
            if self.mkb {
                hit_direction_x = mouse_position.x - self.base().get_position().x;
                hit_direction_y = mouse_position.y - self.base().get_position().y;
            } else {
                hit_direction_x = hit_horiz;
                hit_direction_y = hit_vert;
            }

            let mut hit_direction = Vector2::new(hit_direction_x, hit_direction_y);
            
            hit_direction = hit_direction.normalized();

            let h_box = match &self.hittingbox {
                None => panic!("We should have a hitting box"),
                Some(hb) => hb,
            };
            let balls = h_box.get_overlapping_bodies();
            for ball in balls.iter_shared() {
                if ball.get_class() == "Pong".into() {
                    let mut b = ball.cast::<Pong>();
                    //b.bind_mut().reverse_direction();
                    b.bind_mut().hit_direction(hit_direction);
                }
            }

            let hurt_box = match &self.hurtbox {
                None => panic!("We should have a hurt box"),
                Some(hb) => hb,
            };

            let hazards = hurt_box.get_overlapping_bodies();
            for _hazard in hazards.iter_shared() {
                godot_print!("Hitting here {}", self.base().get_position());
            }
        }
    }
}

#[godot_api]
impl Player {
    #[signal]
    fn col_something();

    #[signal]
    pub fn hit_hazard();

    #[func]
    pub fn reset_player(&mut self) {
        self.base_mut().set_velocity(Vector2::new(0.0, 0.0));
        let position = match &self.start_point {
            None => Vector2::new(0.0, 0.0),
            Some(sp) => sp.get_position()
        };
        self.base_mut().set_position(position);
        self.alive = true;
    }

    #[func]
    pub fn say_hello(&self) {
        godot_print!("hello");
    }
}

impl Player {

    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        if body.get_class() == "Pong".into() {
            self.in_range = true;
        }
    }

    fn on_body_exited(&mut self, body: Gd<Node2D>) {
        if body.get_class() == "Pong".into() {
            self.in_range = false;
        }
    }

    fn on_hazard_entered(&mut self, _body: Gd<Node2D>) {
        godot_print!("Hazard entered!, we need to emit a signal here");
        self.alive = false;
        self.signals().hit_hazard().emit();
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
        if self.jump_counter < self.jump_count {
            self.jump_counter += 1.0 * self.game_speed;
        }  
    }

    fn jump(&mut self, input: &Gd<Input>) {

       if input.is_action_just_pressed("jump") && self.base().is_on_floor() {
           // zero out velocity for new jump
           let mut new_vel = self.base().get_velocity();
           new_vel.y = 0.0;
           self.base_mut().set_velocity(new_vel);
       }
       if input.is_action_pressed("jump") && self.can_jump() {
           let mut new_vel = self.base().get_velocity();
           new_vel.y += self.jump_velocity as f32;
           self.base_mut().set_velocity(new_vel);
           self.tick_jump();
       }

    }

    pub fn update_game_speed(&mut self, speed: f32) {
        self.game_speed = speed;
    }


}
