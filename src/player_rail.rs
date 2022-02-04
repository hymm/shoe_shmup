use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerRail {
    pub rail: Vec<Vec2>,
    // is the rail a closed path
    pub closed: bool,
}

pub enum RailDirection {
    Positive,
    Negative,
}

#[derive(Component)]
pub struct RailPosition {
    /// index of segment
    pub index: usize,
    /// position on current segment, 0.0 -> 1.0
    pub position: f32,
    /// direction of movement
    pub direction: RailDirection,
}

impl RailPosition {
    pub fn next_position(&mut self, rail: &PlayerRail, delta_time: f32, speed: f32) -> Vec2 {
        let index = self.index;
        let segment = rail.rail[index + 1] - rail.rail[index];
        let delta_position = speed * delta_time / segment.length();
        match self.direction {
            RailDirection::Negative => {
                self.position -= delta_position;
                if self.position < 0.0 {
                    self.position = 0.0;
                    self.direction = RailDirection::Positive;
                }
                rail.rail[index] + segment * self.position
            }
            RailDirection::Positive => {
                self.position += delta_position;
                if self.position > 1.0 {
                    self.position = 1.0;
                    self.direction = RailDirection::Negative;
                }
                rail.rail[index] + segment * self.position
            }
        }
    }
}
