use crate::player::Player;
use legion::system;
use lyzah::{
    input::{Input, VirtualKeyCode},
    loader::Loader,
    window::{Window, WindowMode},
    Sprite,
};
use std::f32::consts::PI;
#[system()]
pub fn game_loop(
    #[resource] input: &Input,
    #[resource] game: &mut Game,
    #[resource] window: &Window,
) {
    for key in &input.pressed_keys {
        match key {
            VirtualKeyCode::P => game.pause(),
            VirtualKeyCode::R => game.resume(),
            VirtualKeyCode::F => window.set_window_mode(WindowMode::Fullscreen),
            VirtualKeyCode::M => window.set_window_mode(WindowMode::Windowed),
            VirtualKeyCode::Q => window.close(),
            _ => (),
        }
    }
}

pub struct Game {
    pub is_running: bool,
}

impl Game {
    pub fn new() -> Self {
        Game { is_running: false }
    }

    pub fn start(&mut self, world: &mut legion::World, loader: &mut Loader) {
        self.is_running = true;
        self.create_game_entities(world, loader);
    }

    pub fn pause(&mut self) {
        self.is_running = false;
    }

    pub fn resume(&mut self) {
        self.is_running = true;
    }

    fn create_game_entities(&mut self, world: &mut legion::World, loader: &mut Loader) {
        let mut sprite = Sprite::new(loader.get_texture_id("happy-tree"));
        sprite.set_anchor(0.5, 0.5);
        sprite.set_scale(0.5, 0.5);
        sprite.set_position(100.0, -100.0);
        sprite.set_rotation(-PI / 4.0);

        let mut sprite2 = Sprite::new(loader.get_texture_id("happy-tree"));
        sprite2.set_position(-300.0, -0.0);

        let _ = world.push((sprite, Player()));
        let _ = world.push((sprite2,));
    }
}
