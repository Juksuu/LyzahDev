use lyzah::{Application, Sprite};
use std::path::Path;

fn main() {
    let mut game = Application::new();

    let asset_path = Path::new(env!("OUT_DIR")).join("res");
    let sprite = Sprite::new(asset_path.join("happy-tree.png").to_path_buf());

    game.stage.children.push(sprite);

    game.run();
}
