use crate::prelude::*;

#[system]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &Camera
) {
    let mut batch_draw = DrawBatch::new();
    batch_draw.target(0);

    // weirdly, using .. for the Y axis leaves us with an empty space at the bottom of the window
    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let pt = Point::new(x,y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) {
                let idx = map_index_from_coords(x,y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#')
                };
                batch_draw.set(
                    pt-offset,
                    ColorPair::new(WHITE,BLACK),
                    glyph
                );
            }
        }
    }

    batch_draw.submit(0).expect("Batch error");
}