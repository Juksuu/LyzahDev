use lyzah::{Application, Sprite};
use std::{f32::consts::PI, path::Path};

fn main() {
    let mut game = Application::new();

    let asset_path = Path::new(env!("OUT_DIR")).join("res");
    let mut sprite = Sprite::new(asset_path.join("happy-tree.png").to_path_buf());
    sprite.set_scale(0.5, 0.5);
    sprite.set_position(100.0, -100.0);
    sprite.set_rotation(-PI / 4.0);

    let mut sprite2 = Sprite::new(asset_path.join("happy-tree.png").to_path_buf());
    sprite2.set_position(-300.0, -0.0);

    game.renderables.push(sprite);
    game.renderables.push(sprite2);

    game.run();
}
