use lyzah::{Application, Sprite};
use std::{f32::consts::PI, path::Path};

fn main() {
    let mut game = Application::new();

    let asset_path = Path::new(env!("OUT_DIR")).join("res");
    let mut sprite = Sprite::new(asset_path.join("happy-tree.png").to_path_buf());
    sprite.set_position(100.0, -200.0);
    sprite.set_rotation(-PI / 2.0);

    let mut sprite2 = Sprite::new(asset_path.join("happy-tree.png").to_path_buf());
    sprite2.set_position(-300.0, -200.0);

    game.stage.children.push(sprite);
    game.stage.children.push(sprite2);

    game.run();
}
