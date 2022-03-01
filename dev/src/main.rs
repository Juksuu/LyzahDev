use lyzah::{ecs::schedule::SystemStage, loader, Application};

mod game;
mod player;

use game::Game;

fn main() {
    let game_stage = SystemStage::single(game::game_loop);
    let stage = SystemStage::parallel().with_system(player::move_player);

    let mut app = Application::builder()
        .add_stage("game_loop", game_stage)
        .add_stage("update", stage)
        .build();

    let images = vec![loader::ResourceData {
        name: "happy-tree",
        path: "dev/res/happy-tree.png",
    }];

    app.loader.load_images(images);

    let mut game = Game::new();
    game.start(&mut app.world, &mut app.loader);

    app.world.insert_resource(game);

    app.run();
}
