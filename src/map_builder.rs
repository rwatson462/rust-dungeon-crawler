use crate::prelude::*;

const NUM_ROOMS: usize = 20;


pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point
}

impl MapBuilder {
    pub fn new(map: Map) -> Self {
        MapBuilder {
            map,
            rooms: vec![],
            player_start: Point::zero()
        }
    }

    pub fn build_map(&mut self) {
        self.fill(TileType::Wall);

        let mut rng = RandomNumberGenerator::new();
        self.build_random_rooms(&mut rng);
        self.set_player_start();
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH-10),
                rng.range(1, SCREEN_HEIGHT-10),
                rng.range(2, 10),
                rng.range(2, 10)
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                // loop over each point in the room Rect and set it's corresponding point on the map
                // to a Floor tile
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_index_from_coords(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    fn set_player_start(&mut self) {
        // find the room in the top left corner of the map
        // set the player's start position to the center(ish) of that room
        let mut best_room = self.rooms[0];
        for room in self.rooms.iter() {
            if room.center().x <= best_room.center().x && room.center().y <= best_room.center().y {
                best_room = *room;
            }
        }

        self.player_start = best_room.center();
    }
}