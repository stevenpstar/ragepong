use godot::{builtin::{Array, Vector2}, classes::{ILine2D, Line2D}, obj::{Base, Gd, WithBaseField}, prelude::{godot_api, GodotClass}};

use crate::player::pong::Pong;

#[derive(GodotClass)]
#[class(base=Line2D)]
pub struct BallTrail {
    #[export]
    pong: Option<Gd<Pong>>,
    points: Array<Vector2>,
    max_points: usize,
    base: Base<Line2D>,
}

#[godot_api]
impl ILine2D for BallTrail {
    fn init(base: Base<Line2D>) -> Self {
        Self {
            pong: None,
            points: Default::default(),
            max_points: 40,
            base,
        }
    }

    fn process(&mut self, _delta: f64) {
        let pong_position = match &self.pong {
            None => panic!("No pong attached to line!"),
            Some(p) => p.get_global_position(),
        };

        self.points.push_front(pong_position);

        if self.points.len() > self.max_points {
            self.points.pop();
        }

        self.base_mut().clear_points();
        let current_points = self.points.clone();
        for point in current_points.iter_shared() {
            self.base_mut().add_point(point);
        }
    }
}
