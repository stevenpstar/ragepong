use godot::{builtin::Array, classes::{INode, Input, Node, Node2D}, global::godot_print, obj::{Base, Gd, WithBaseField}, prelude::{godot_api, GodotClass}};

use crate::player::pong::Pong;
use crate::player::player::Player;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameState {
    input: Gd<Input>,
    #[export]
    player: Option<Gd<Player>>,
    #[export]
    balls: Array<Gd<Pong>>,
    #[export]
    player_start: Option<Gd<Node2D>>,
    #[export]
    pong_start: Option<Gd<Node2D>>,
    #[export]
    gamespeed: f64,
    base: Base<Node>,
}

#[godot_api]
impl INode for GameState {
    fn init(base: Base<Node>) -> Self {
        Self {
            input: Input::singleton(),
            player: None,
            balls: Default::default(),
            player_start: None,
            pong_start: None,
            gamespeed: 1.0,
            base,
        }
    }

    fn ready(&mut self) {
        let this = self.to_gd();
        let player = match &mut self.player {
            None => panic!("Heyoheyo"),
            Some(p) => p,
        };

        player.signals()
            .hit_hazard()
            .connect_obj(&this, |s: &mut Self| {
                s.reset_game();
            });
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.input.is_action_pressed("shoot") {
            self.set_gamestate_speed(0.5);
        } else if self.input.is_action_just_released("shoot") {
            self.set_gamestate_speed(1.0);
        }
    }
}

#[godot_api]
impl GameState {
    #[func]
    pub fn set_gamestate_speed(&mut self, speed: f64) {
        self.gamespeed = speed;
        for mut ball in self.balls.iter_shared()  {
            ball.bind_mut().update_game_speed(self.gamespeed as f32);
        }
        match &mut self.player {
            None => panic!("No player ref found"),
            Some(p) => p.bind_mut().update_game_speed(self.gamespeed as f32)
        };

    }

    pub fn reset_game(&mut self) {
        match &mut self.player {
            None => panic!("Player not found"),
            Some(player) => player.bind_mut().reset_player()
        };
        for mut pong in self.balls.iter_shared() {
            pong.bind_mut().reset();
            godot_print!("heyo")
        }
    }
}


