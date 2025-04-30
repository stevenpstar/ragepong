use godot::{builtin::Array, classes::{INode, Input, Node}, obj::{Base, Gd}, prelude::{godot_api, GodotClass}};

use crate::player::pong::Pong;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameState {
    input: Gd<Input>,
    #[export]
    balls: Array<Gd<Pong>>,
    #[export]
    gamespeed: f64,
    base: Base<Node>,
}

#[godot_api]
impl INode for GameState {
    fn init(base: Base<Node>) -> Self {
        Self {
            input: Input::singleton(),
            balls: Default::default(),
            gamespeed: 1.0,
            base,
        }
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
    }
}


