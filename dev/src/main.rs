use legion::*;
use lyzah::{Application, Sprite};
use std::{f32::consts::PI, path::Path};

#[system(for_each)]
fn update_positions(sprite: &mut Sprite) {
    sprite.set_rotation(sprite.rotation + 0.01)
}

fn main() {
    let mut game = Application::new();

    let asset_path = Path::new(env!("OUT_DIR")).join("res");
    let images = vec![asset_path.join("happy-tree.png").to_path_buf()];
    game.resources.load_images(images);

    let mut world = World::default();
    let mut resources = Resources::default();

    let mut sprite = Sprite::new(game.resources.get("happy-tree.png".to_string()));
    sprite.set_anchor(0.5, 0.5);
    sprite.set_scale(0.5, 0.5);
    sprite.set_position(100.0, -100.0);
    sprite.set_rotation(-PI / 4.0);

    let _entity = world.push((sprite,));

    let mut sprite2 = Sprite::new(game.resources.get("happy-tree.png".to_string()));
    sprite2.set_position(-300.0, -0.0);

    let _entity2 = world.push((sprite2,));

    let mut schedule = Schedule::builder()
        .add_system(update_positions_system())
        .build();

    game.run(move || {
        schedule.execute(&mut world, &mut resources);

        // you define a query be declaring what components you want to find, and how you will access them
        let mut query = <&Sprite>::query();

        // you can then iterate through the components found in the world
        query
            .iter(&world)
            .map(|s| (s.texture_id, s.get_raw_instance()))
            .collect()
    });
}
