use godot::{builtin::{Array, GString}, classes::{ INode, Input, Node, Node2D, PackedScene, ResourceLoader}, global::{godot_error, godot_print}, obj::{Base, Gd, WithBaseField}, prelude::{godot_api, GodotClass}};

use crate::player::pong::Pong;
use crate::player::player::Player;
use crate::core::level::Level;

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
    #[export]
    levels: Array<GString>,
    #[export]
    base_path: GString,
    #[export]
    level_str: GString,
    current_level: Option<Gd<Level>>,
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
            levels: Default::default(),
            base_path: Default::default(),
            level_str: Default::default(),
            current_level: None,
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
        let base_path = self.base_path.clone();
        let level_str = self.level_str.clone();
        let level_path: GString = format!("{base_path}{level_str}").into();
        self.change_level(level_path.to_string());
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.input.is_action_pressed("shoot") {
            self.set_gamestate_speed(0.5);
        } else if self.input.is_action_just_released("shoot") {
            self.set_gamestate_speed(1.0);
        }

        if self.input.is_action_just_pressed("change_level") {
            self.change_level("res://Levels/level_1.tscn".to_string());
        }
    }
}

#[godot_api]
impl GameState {
    #[signal]
    fn sigsig();

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
        let level = match &self.current_level {
            None => {
                godot_print!("No current level in gamestate");
                panic!("no current level in gamestate");
            },
            Some(lvl) => lvl
        };
        level.bind().reset_obstacles();

        match &mut self.player {
            None => panic!("Player not found"),
            Some(player) => player.bind_mut().reset_player()
        };
        for mut pong in self.balls.iter_shared() {
            pong.bind_mut().reset();
            godot_print!("heyo")
        }
    }

    fn change_level(&mut self, level_path: String) {
        godot_print!("Trying to change level!");
        let this = self.to_gd();

        if self.base().get_child_count() > 0 {
            let level = self.base_mut().get_child(0).unwrap();
            level.cast::<Node2D>().queue_free();
        }

        let mut res_loader = ResourceLoader::singleton();

        let mut next_level: Gd<Level> = res_loader.load(&level_path)
            .unwrap().cast::<PackedScene>().instantiate_as::<Level>();
        self.base_mut().add_child(&next_level);

        {
            let p_start = match &mut self.player_start {
                None => {
                    godot_error!("No player start in gamestate");
                    panic!("uh oh!")
                },
                Some(ps) => ps
            };

            p_start.set_position(next_level.bind().get_player_start_position());

        }

        {
            let pong_start = match &mut self.pong_start {
                None => {
                    godot_error!("No pong start in gamestate");
                    panic!("uh oh!")
                },
                Some(ps) => ps
            };

            pong_start.set_position(next_level.bind().get_pong_start_position());

        }
    
        let mut level_opt = next_level.bind_mut().get_level_end();
        let level_end = match &mut level_opt {
            None => {
                godot_print!("No level end on level found");
                panic!("No level end on level found");
            },
            Some(end) => end
        };

        {
            for mut pong in &mut self.balls.iter_shared() {
                pong.bind_mut().set_start_dir(next_level.bind().get_pong_direction());
            }
        }


        level_end.signals()
            .level_ended()
            .connect_obj(&this, |s: &mut Self, _path| {
                s.change_level(_path);
        });
        self.current_level = Some(next_level);
        self.reset_game();
    }
}


