use lyzah::{
    ecs::{
        schedule::SystemStage,
        system::{Commands, Query, Res},
    },
    input::Input,
    loader::{self, Loader},
    Application, Sprite,
};
use std::f32::consts::PI;

mod game;
mod player;

use game::Game;

fn add_sprites(mut commands: Commands, loader: Res<Loader>, input: Res<Input>) {
    if input.is_mouse_inside_window {
        let mut sprite = Sprite::new(loader.get_texture_id("kotbulla"));
        sprite.set_anchor(0.5, 0.5);
        sprite.set_scale(0.5, 0.5);
        sprite.set_position(100.0, -100.0);
        sprite.set_rotation(-PI / 4.0);
        sprite.set_position(input.mouse_pos.x, input.mouse_pos.y);

        commands.spawn().insert(sprite);
    }
}

fn rotate_sprites(mut query: Query<&mut Sprite>) {
    for mut sprite in query.iter_mut() {
        let old_rotation = sprite.rotation;
        sprite.set_rotation(old_rotation + PI / 2.0);
    }
}

fn main() {
    let game_stage = SystemStage::single(game::game_loop);
    let stage = SystemStage::parallel()
        .with_system(player::move_player)
        .with_system(rotate_sprites)
        .with_system(add_sprites);

    let mut app = Application::builder()
        .add_stage("game_loop", game_stage)
        .add_stage("update", stage)
        .build();

    {
        let mut loader = app.world.get_resource_mut::<Loader>().unwrap();
        let images = vec![
            loader::ResourceData {
                name: "happy-tree",
                path: "dev/res/happy-tree.png",
            },
            loader::ResourceData {
                name: "kotbulla",
                path: "dev/res/kotbulla.png",
            },
        ];

        loader.load_images(images);
    }

    let mut game = Game::new();
    game.start(&mut app.world);

    app.world.insert_resource(game);

    app.run();
}
