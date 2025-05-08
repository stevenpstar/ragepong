use core::panic;

use godot::{builtin::{Color, Vector2}, classes::{ AnimatedSprite2D, Area2D, CharacterBody2D, ICharacterBody2D, Input, Line2D, Node2D }, global::{godot_print, move_toward}, obj::{Base, Gd, WithBaseField, WithUserSignals}, prelude::{godot_api, GodotClass}};

use crate::{core::{colour_component::ColourComponent, colours::Colour}, obstacles::{laser_gate::LaserGate, switch::Switch}, player::pong::Pong};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    input: Gd<Input>,
    face_right: bool,
    alive: bool,
    aiming: bool,
    in_range: bool,
    can_aim: bool,
    has_jumped: bool,
    has_dashed: bool,
    is_dashing: bool,
    slow_broken: bool,
    on_ball: bool,
    ball: Option<Gd<Pong>>,
    dash_direction: Vector2,
    #[export]
    can_double_jump: bool,
    jump_count: i32,
    #[export]
    speed: f64,
    #[export]
    max_speed: f64,
    game_speed: f32,
    #[export]
    jump_velocity: f64,
    #[export]
    jump_timer: f32,
    #[export]
    jump_time: f32,
    #[export]
    dash_velocity: f32,
    #[export]
    dash_timer: f32,
    #[export]
    dash_time: f32,
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
    #[export]
    colour_component: Option<Gd<ColourComponent>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {

    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Player Loaded (from rust!)");

        Self {
            input: Input::singleton(),
            face_right: true,
            alive: true,
            aiming: false,
            in_range: false,
            can_aim: true,
            has_jumped: false,
            has_dashed: false,
            is_dashing: false,
            slow_broken: false,
            on_ball: false,
            ball: None,
            dash_direction: Vector2::new(0.0, 0.0),
            can_double_jump: false,
            jump_count: 0,
            speed: 50.0,
            max_speed: 300.0,
            game_speed: 1.0,
            jump_velocity: -200.0,
            jump_time: 4.0,
            jump_timer: 0.0, 
            dash_velocity: 200.0,
            dash_timer: 0.0,
            dash_time: 4.0,
            mkb: true, // defaults to player using mouse/key board for aiming. Changes dynamically.
            hittingbox: None,
            hurtbox: None,
            sprite: None,
            start_point: None,
            aim_ind: None,
            aim_ind_in_range: None,
            colour_component: None,
            base,
        }
    }

    fn ready(&mut self) {
        let this = self.to_gd();
        let col = &self.get_colour();
        self.set_player_colour(col);

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

        hurt.signals()
            .body_exited()
            .connect_obj(&this, |s: &mut Self, body| {
                s.on_hazard_exited(body);
            });

        hurt.signals()
            .area_entered()
            .connect_obj(&this, |s: &mut Self, area| {
                s.on_hazard_area_entered(area);
            });

        hurt.signals()
            .area_exited()
            .connect_obj(&this, |s: &mut Self, area| {
                s.on_hazard_area_exited(area);
            });


        // set player spawn
        self.reset_player();
    }

    fn process(&mut self, _delta: f64) {
        if !self.alive {
            return;
        }

        let direction = self.input.get_axis("move_left", "move_right");

        let on_floor = self.base().is_on_floor();
        let vel_y = self.base().get_velocity().y;
        if direction != 0.0 {
            self.face_right = direction > 0.0;
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

    }

    fn physics_process(&mut self, delta: f64) {

        if !self.alive {
            return;
        }
        
        let input = Input::singleton();

        if !self.base().is_on_floor() {
            let mut new_velocity = self.base().get_velocity();
            let mut gravity_slow = 0.0;

            if self.has_jumped && !self.has_dashed && self.base().get_velocity().y > 0.0 {
                gravity_slow = 500.0;
                if input.is_action_pressed("move_down") {
                    gravity_slow = -gravity_slow;
                }
            }
            new_velocity.x += (self.base().get_gravity().x - gravity_slow) * delta as f32;
            new_velocity.y += (self.base().get_gravity().y - gravity_slow) * delta as f32;
            self.base_mut().set_velocity(new_velocity);
        } else {
            // Reset jump timer whenever we hit the floor
            self.jump_timer = 0.0;
            self.jump_count = 0;
            self.dash_timer = 0.0;
            self.has_jumped = false;
            self.has_dashed = false;
        }

        self.jump(&input);

        let direction = input.get_axis("move_left", "move_right");
        let hit_vert = input.get_axis("hit_up", "hit_down");
        let hit_horiz = input.get_axis("hit_left", "hit_right");

        if self.aiming {
            self.aim(hit_horiz, hit_vert);
        }


        if direction != 0.0 && !input.is_action_pressed("aim") {
            let max_speed = self.max_speed as f32 * direction;
            let mut vel_x = self.base().get_velocity().x;

            // reset to zero if changing direction
            if direction < 0.0 && self.base().get_velocity().x > 0.2 {
                vel_x = 0.0;
            } else if direction > 0.0 && self.base().get_velocity().x < -0.2 {
                vel_x = 0.0;
            }
            
            vel_x += direction * self.speed as f32 * self.game_speed as f32;
            let vel_y = self.base().get_velocity().y as f32;
            if max_speed > 0.0 && vel_x > max_speed{
                vel_x = max_speed as f32;
            } else if max_speed < 0.0 && vel_x < max_speed as f32 {
                vel_x = max_speed as f32;
            }
            self.base_mut().set_velocity(Vector2::new(vel_x, vel_y));
        } else {
            let vel_x = move_toward(self.base().get_velocity().x as f64, 0.0, self.speed * self.game_speed as f64);
            let vel_y = self.base().get_velocity().y as f32;
            self.base_mut().set_velocity(Vector2::new(vel_x as f32, vel_y));
        }

        // dashing
        if input.is_action_just_pressed("dash") && !self.has_dashed && self.can_dash() && self.aiming {
            self.slow_broken = true;
            self.signals().break_slow().emit();
            self.has_dashed = true;
            self.is_dashing = true;
            let vel_x = hit_horiz * self.dash_velocity;
            let vel_y = hit_vert * self.dash_velocity;
            self.dash_direction = Vector2::new(vel_x, vel_y);
            self.base_mut().set_velocity(Vector2::new(vel_x, vel_y));
        } else if input.is_action_pressed("dash") && self.has_dashed && self.can_dash() {
            let vel_x = self.dash_direction.x;
            let vel_y = self.dash_direction.y;

            self.base_mut().set_velocity(Vector2::new(vel_x, vel_y));
            self.tick_dash();
        } else if input.is_action_just_released("dash") {
            self.is_dashing = false;
            let mut new_vel = self.base().get_velocity();
            new_vel.y = new_vel.y / 2.0;
            self.base_mut().set_velocity(new_vel);
        }

        {
            let mut new_vel = self.base().get_velocity();
            new_vel.x = new_vel.x * self.game_speed as f32;
            new_vel.y = new_vel.y * self.game_speed as f32;
            if self.on_ball {
                let ball = match &self.ball {
                    None => panic!("Ball not found!"),
                    Some(b) => b
                };
                new_vel.y = 0.0 + -ball.bind().get_pong_speed() as f32 * 1.7 as f32;
            }

            self.base_mut().set_velocity(new_vel);
        }

        self.base_mut().move_and_slide();

        if input.is_action_just_pressed("aim") && self.can_aim {
            self.slow_broken = false;
            self.can_aim = false;
            self.aiming = true;
        }

        if input.is_action_pressed("aim") && self.aiming && !self.slow_broken {
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

        if input.is_action_just_pressed("shoot") && self.aiming
        {

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
                    let ball_col = b.bind_mut().get_colour();
                    b.bind_mut().unlock();
                    b.bind_mut().hit_direction(hit_direction);
                    self.set_player_colour(&ball_col);
                } 
            }

        }

        if (input.is_action_just_released("aim") && !self.can_aim) || self.slow_broken {
            self.can_aim = true;
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

        }

        if input.is_action_just_pressed("interact") {

            let h_box = match &self.hittingbox {
                None => panic!("We should have a hitting box"),
                Some(hb) => hb,
            };

            let interactives = h_box.get_overlapping_areas();
            for int in interactives.iter_shared() {
                if int.get_class() == "Switch".into() {
                    godot_print!("Switch hit");
                    let mut s = int.cast::<Switch>();
                    s.bind_mut().toggle();
                }
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

    #[signal]
    pub fn break_slow();

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

    fn on_hazard_entered(&mut self, body: Gd<Node2D>) {

        let parent = match body.get_parent() {
            None => return, // ignore
            Some(p) => p
        };

        if parent.get_class() == "Pong".into() {
            return;
        }

        self.kill();
    }

    fn on_hazard_exited(&mut self, _body: Gd<Node2D>) {
       // nothing
    }

    fn on_hazard_area_entered(&mut self, area: Gd<Area2D>) {
        let parent = match area.get_parent() {
            None => return, // ignore
            Some(p) => p
        };

        if parent.get_class() == "Pong".into() {
            self.on_ball = true;
            let pong = parent.try_cast::<Pong>().expect("Should be able to cast to Pong");
            self.ball = Some(pong);
            return;
        }

        if area.get_class() == "LaserGate".into() {
            let mut gate = area.cast::<LaserGate>();
            let gate_col = gate.bind_mut().get_colour();
            let player_col = self.get_colour();
            if gate.bind().get_is_open() == false && gate_col != player_col {
                self.kill();
            } else if gate.bind().get_is_open() == false && gate_col == player_col {
                godot_print!("Resetting dash and jump");
                self.has_jumped = false;
                self.jump_count = 0;
                self.jump_timer = 0.0;
                self.has_dashed = false;
                self.dash_timer = 0.0;
            }
        } else {
            self.kill();
        }
    }

    fn on_hazard_area_exited(&mut self, area: Gd<Area2D>) {
        let parent = match area.get_parent() {
            None => return, // ignore
            Some(p) => p
        };

        if parent.get_class() == "Pong".into() {
            self.on_ball = false;
            self.ball = None;
        }
    }

    fn can_jump(&mut self) -> bool {
        if self.base().is_on_floor() {
            return true;
        }
        if self.jump_timer < self.jump_time {
            return true;
        }
        return false;
    }

    fn tick_jump(&mut self) {
        if self.jump_timer < self.jump_time {
            self.jump_timer += 1.0 * self.game_speed;
        }  
    }

    fn can_dash(&mut self) -> bool {
        if self.has_dashed {
            return false;
        }
        if self.base().is_on_floor() {
            return true;
        }
        if self.dash_timer < self.dash_time {
            return true;
        }
        return false;
    }


    fn tick_dash(&mut self) {
        if self.dash_timer < self.dash_time {
            self.dash_timer += 1.0 * self.game_speed;
        } else {
            self.is_dashing = false;
            let mut new_vel = self.base().get_velocity();
            new_vel.y = new_vel.y / 2.0;
            self.base_mut().set_velocity(new_vel);
        }
    }


    fn jump(&mut self, input: &Gd<Input>) {

       if input.is_action_just_pressed("jump") && (self.base().is_on_floor() || self.can_dbl_jump()) {
           self.jump_timer = 0.0;
           // zero out velocity for new jump
           let mut new_vel = self.base().get_velocity();
           new_vel.y = self.jump_velocity as f32 * 2.0 as f32;
           self.base_mut().set_velocity(new_vel);
           self.has_jumped = true;
           self.jump_count += 1;
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

    pub fn kill(&mut self) {
        self.alive = false;
        self.signals().hit_hazard().emit();
    }

    fn can_dbl_jump(&mut self) -> bool {
        if self.can_double_jump && self.jump_count < 2 {
            self.jump_count += 1;
            return true;
        }
        return false;
    }

    fn aim(&mut self, hit_horiz: f32, hit_vert: f32) {
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

    pub fn get_colour(&mut self) -> Colour {
        let colour = match &self.colour_component {
            None => {
                godot_print!("No colour component on player!");
                panic!("No colour component on player!");
            },
            Some(cc) => cc
        };
        return colour.bind().get_obj_colour();
    }

    fn set_player_colour(&mut self, colour: &Colour) {

        let colour_comp = match &mut self.colour_component {
            None => {
                godot_print!("Player should have a colour component");
                panic!("No colour component for player!");
            },
            Some(cc) => cc
        };

        colour_comp.bind_mut().set_obj_colour(colour);

        let sprite = match &mut self.sprite {
            None => {
                godot_print!("Player should have a character sprite");
                panic!("No character sprite for player!");
            },
            Some(spr) => spr
        };

        match colour {
            Colour::White => sprite.set_modulate(Color::from_rgb(1.0, 1.0, 1.0)),
            Colour::Red => sprite.set_modulate(Color::from_rgb(1.0, 0.0, 0.0)),
            Colour::Blue => sprite.set_modulate(Color::from_rgb(0.0, 0.0, 1.0)),
            Colour::Green => sprite.set_modulate(Color::from_rgb(0.0, 1.0, 0.0)),
        };
    }



}
