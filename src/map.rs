use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;


#[derive(Copy,Clone,PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles: Vec<TileType>
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_index_from_coords(x,y);
                match self.tiles[idx] {
                    TileType::Floor => ctx.set(x,y, YELLOW, BLACK, to_cp437(0 as char)),
                    TileType::Wall => ctx.set(x,y, GREEN, BLACK, to_cp437('#')),
                }
            }
        }
    }
}


pub fn map_index_from_coords(x: i32, y: i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}