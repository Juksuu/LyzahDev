use lyzah::{loader, Application};

mod game;
mod player;

use game::Game;

fn main() {
    let mut app = Application::builder()
        .with_system(player::move_player_system())
        .with_system(game::game_loop_system())
        .build();

    let images = vec![loader::ResourceData {
        name: "happy-tree",
        path: "dev/res/happy-tree.png",
    }];

    app.loader.load_images(images);

    let mut game = Game::new();
    game.start(&mut app.world, &mut app.loader);

    app.resources.insert(game);

    app.run();
}
