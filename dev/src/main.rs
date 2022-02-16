use game::Game;
use legion::*;
use lyzah::Application;
use std::path::Path;

mod game;
mod player;

fn main() {
    let mut app = Application::new();

    let asset_path = Path::new(env!("OUT_DIR")).join("res");
    let images = vec![asset_path.join("happy-tree.png").to_path_buf()];
    app.resources.load_images(images);

    let mut world = World::default();
    let mut resources = Resources::default();

    let mut game = Game::new();
    game.start(&mut world, &mut app.resources);

    resources.insert(game);

    let mut schedule = Schedule::builder()
        .add_system(player::move_player_system())
        .build();

    app.run(world, resources, move |mut world, mut resources| {
        schedule.execute(&mut world, &mut resources);
    });
}
