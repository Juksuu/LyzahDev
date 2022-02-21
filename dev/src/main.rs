use game::Game;
use lyzah::Application;
use std::path::Path;

mod game;
mod player;

fn main() {
    let mut app = Application::builder()
        .with_system(player::move_player_system())
        .build();

    let asset_path = Path::new(env!("OUT_DIR")).join("res");
    let images = vec![asset_path.join("happy-tree.png").to_path_buf()];

    app.loader.load_images(images);

    let mut game = Game::new();
    game.start(&mut app.world, &mut app.loader);

    app.resources.insert(game);

    app.run();
}
