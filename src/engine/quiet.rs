use super::super::game::GameState;
use super::Engine;

pub struct Quiet;

impl Engine for Quiet {
    fn update(&mut self, _state: &GameState) {}
}
