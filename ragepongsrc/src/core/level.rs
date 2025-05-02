use godot::{classes::{INode2D, Node2D}, obj::{Base, Gd}, prelude::{godot_api, GodotClass}};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Level {
    #[export]
    player_start: Option<Gd<Node2D>>,
    #[export]
    pong_start: Option<Gd<Node2D>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Level {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            player_start: None,
            pong_start: None,
            base,
        }
    }
}
