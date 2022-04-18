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
        use std::cmp::{min,max};
        // bind map to screen
        let min_x = max(0, player_position.x - DISPLAY_WIDTH/2);
        if min_x == 0 {
            self.left_x = min_x;
            self.right_x = min_x + DISPLAY_WIDTH;
        } else {
            let max_x = min(SCREEN_WIDTH, player_position.x + DISPLAY_WIDTH/2);
            if max_x == player_position.x + DISPLAY_WIDTH/2 {
                self.right_x = max_x;
                self.left_x = max_x - DISPLAY_WIDTH;
            }
        }

        let min_y = max(0, player_position.y - DISPLAY_HEIGHT/2);
        if min_y == 0 {
            self.top_y = min_y;
            self.bottom_y = min_y + DISPLAY_HEIGHT;
        } else {
            let max_y = min(SCREEN_HEIGHT, player_position.y + DISPLAY_HEIGHT/2);
            if max_y == player_position.y + DISPLAY_HEIGHT/2 {
                self.bottom_y = max_y;
                self.top_y = max_y - DISPLAY_HEIGHT;
            }
        }
    }
}