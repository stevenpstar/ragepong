use godot::{builtin::{Array, Vector2}, classes::{INode2D, Node2D}, obj::{Base, Gd}, prelude::{godot_api, GodotClass}};

use crate::obstacles::laser_gate::LaserGate;

use super::{camera_area::CameraArea, colours::Colour, level_end::LevelEnd};

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
    white_pong_start: Option<Gd<Node2D>>,
    #[export]
    blue_pong_start: Option<Gd<Node2D>>,
    #[export]
    red_pong_start: Option<Gd<Node2D>>,
    #[export]
    green_pong_start: Option<Gd<Node2D>>,
    #[export]
    white_pong_direction: Vector2,
    #[export]
    blue_pong_direction: Vector2,
    #[export]
    red_pong_direction: Vector2,
    #[export]
    green_pong_direction: Vector2,
    #[export] 
    starting_cam: Option<Gd<CameraArea>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Level {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            obstacles: Default::default(),
            level_end: None,
            player_start: None,
            white_pong_start: None,
            blue_pong_start: None,
            red_pong_start: None,
            green_pong_start: None,
            white_pong_direction: Vector2::new(0.0, 0.0),
            blue_pong_direction: Vector2::new(0.0, 0.0),
            red_pong_direction: Vector2::new(0.0, 0.0),
            green_pong_direction: Vector2::new(0.0, 0.0),
            starting_cam: None,
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
    pub fn get_pong_start_position(&self, colour: String) -> Vector2 {

        if colour == "white" {
            let pong_start = match &self.white_pong_start {
                None => Vector2::new(0.0, 0.0),
                Some(start) => start.get_position()
            };

            return pong_start;
        }
        else if colour == "blue" {
            let pong_start = match &self.blue_pong_start {
                None => Vector2::new(0.0, 0.0),
                Some(start) => start.get_position()
            };

            return pong_start;
        }
        else if colour == "red" {
            let pong_start = match &self.red_pong_start {
                None => Vector2::new(0.0, 0.0),
                Some(start) => start.get_position()
            };

            return pong_start;
        } else {
            let pong_start = match &self.green_pong_start {
                None => Vector2::new(0.0, 0.0),
                Some(start) => start.get_position()
            };

            return pong_start;
        }
    }

    pub fn get_pong_direction<'b>(&self, colour: &'b Colour) -> Vector2 {
        match colour {
            Colour::White => self.white_pong_direction,
            Colour::Blue => self.blue_pong_direction,
            Colour::Red => self.red_pong_direction,
            Colour::Green => self.green_pong_direction,
        }
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

    #[func]
    pub fn get_starting_camera(&self) -> Vector2 {
        let cam = match &self.starting_cam {
            None => Vector2::new(0.0, 0.0),
            Some(c) => c.get_position()
        };

        cam
    }
}
