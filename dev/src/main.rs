use legion::*;
use lyzah::{
    input::{Input, VirtualKeyCode},
    Application, Sprite, Time,
};
use std::{f32::consts::PI, path::Path};

struct Player();

#[system(par_for_each)]
fn update_rotations(sprite: &mut Sprite, #[resource] time: &Time) {
    let amount = PI * time.delta_time.as_secs_f32();
    sprite.set_rotation(sprite.rotation + amount);

    // println!("{:?}, {:?}", time.elapsed, time.delta_time)
}

#[system(par_for_each)]
fn move_player(
    sprite: &mut Sprite,
    _player: &Player,
    #[resource] time: &Time,
    #[resource] input: &Input,
) {
    let old_pos = sprite.position;
    let move_amount = 200.0 * time.delta_time.as_secs_f32();
    let mut pos_change = (old_pos.0, 0.0);

    for key in &input.pressed_keys {
        match key {
            VirtualKeyCode::A => new_pos.0 -= move_amount,
            VirtualKeyCode::S => new_pos.1 -= move_amount,
            VirtualKeyCode::W => new_pos.1 += move_amount,
            VirtualKeyCode::D => new_pos.0 += move_amount,
            _ => (),
        }
    }

    sprite.set_position(old_pos.x + pos_change.0, old_pos.y + pos_change.1);
}

fn main() {
    let mut game = Application::new();

    let asset_path = Path::new(env!("OUT_DIR")).join("res");
    let images = vec![asset_path.join("happy-tree.png").to_path_buf()];
    game.resources.load_images(images);

    let mut world = World::default();
    let resources = Resources::default();

    let mut sprite = Sprite::new(game.resources.get("happy-tree.png".to_string()));
    sprite.set_anchor(0.5, 0.5);
    sprite.set_scale(0.5, 0.5);
    sprite.set_position(100.0, -100.0);
    sprite.set_rotation(-PI / 4.0);

    let _entity = world.push((sprite, Player()));

    let mut sprite2 = Sprite::new(game.resources.get("happy-tree.png".to_string()));
    sprite2.set_position(-300.0, -0.0);

    let _entity2 = world.push((sprite2,));

    let mut schedule = Schedule::builder()
        .add_system(update_rotations_system())
        .add_system(move_player_system())
        .build();

    game.run(world, resources, move |mut world, mut resources| {
        schedule.execute(&mut world, &mut resources);
    });
}
