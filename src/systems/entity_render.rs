use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera
) {
    let mut batch_draw = DrawBatch::new();
    batch_draw.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);
    
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos,render)| {
            batch_draw.set(
                *pos-offset,
                render.colour,
                render.glyph
            );
        }
    );

    // 5000 is roughly the map size plus a bit of space (map is around 4k sprites)
    batch_draw.submit(5000).expect("Batch error");
}