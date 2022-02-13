use legion::*;
use lyzah::{Application, Sprite, Time};
use std::{f32::consts::PI, path::Path};

#[system(par_for_each)]
fn update_positions(sprite: &mut Sprite, #[resource] time: &Time) {
    let amount = PI * time.delta_time.as_secs_f32();
    sprite.set_rotation(sprite.rotation + amount)
}

fn main() {
    let mut game = Application::new();

    let asset_path = Path::new(env!("OUT_DIR")).join("res");
    let images = vec![asset_path.join("happy-tree.png").to_path_buf()];
    game.resources.load_images(images);

    let mut world = World::default();
    let mut resources = Resources::default();

    resources.insert(Time::default());

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

    game.run(world, resources, move |mut world, mut resources| {
        schedule.execute(&mut world, &mut resources);
    });
}
