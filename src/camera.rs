use crate::prelude::*;

pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        Self {
            left_x: player_position.x - DISPLAY_WIDTH/2,
            right_x: player_position.x + DISPLAY_WIDTH/2,
            top_y: player_position.y - DISPLAY_HEIGHT/2,
            bottom_y: player_position.y + DISPLAY_HEIGHT/2
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        self.left_x = player_position.x - DISPLAY_WIDTH/2;
        self.right_x = self.left_x + DISPLAY_WIDTH;

        self.top_y = player_position.y - DISPLAY_HEIGHT/2;
        self.bottom_y = self.top_y + DISPLAY_HEIGHT;
    }
}