use crate::player::Player;
use legion::World;
use lyzah::{Resources, Sprite, Texture};
use std::f32::consts::PI;

pub struct Game {
    pub is_running: bool,
}

impl Game {
    pub fn new() -> Self {
        Game { is_running: false }
    }

    pub fn start(&mut self, world: &mut World, resources: &mut Resources) {
        self.is_running = true;
        create_game_entities(world, resources);
    }

    pub fn pause(&mut self) {
        self.is_running = false;
    }

    pub fn resume(&mut self) {
        self.is_running = true;
    }
}

fn create_game_entities(world: &mut World, resources: &mut Resources) {
    let mut sprite = Sprite::new(resources.get::<Texture>("happy-tree.png".to_string()));
    sprite.set_anchor(0.5, 0.5);
    sprite.set_scale(0.5, 0.5);
    sprite.set_position(100.0, -100.0);
    sprite.set_rotation(-PI / 4.0);

    let mut sprite2 = Sprite::new(resources.get::<Texture>("happy-tree.png".to_string()));
    sprite2.set_position(-300.0, -0.0);

    let _ = world.push((sprite, Player()));
    let _ = world.push((sprite2,));
}
