use crate::player::Player;
use lyzah::{
    ecs::{
        system::{Res, ResMut},
        world::World,
    },
    input::{Input, VirtualKeyCode},
    loader::Loader,
    window::{Window, WindowMode},
    Sprite, Text,
};
use std::f32::consts::PI;

pub fn game_loop(input: Res<Input>, mut game: ResMut<Game>, window: Res<Window>) {
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

    pub fn start(&mut self, world: &mut World) {
        self.is_running = true;
        self.create_game_entities(world);
    }

    pub fn pause(&mut self) {
        self.is_running = false;
    }

    pub fn resume(&mut self) {
        self.is_running = true;
    }

    fn create_game_entities(&mut self, world: &mut World) {
        let loader = world.get_resource::<Loader>().unwrap();
        let mut sprite = Sprite::new(loader.get_resource_id("kotbulla"));
        sprite.set_anchor(0.5, 0.5);
        sprite.set_scale(0.5, 0.5);
        sprite.set_position(100.0, -100.0);
        sprite.set_rotation(-PI / 4.0);

        let mut sprite2 = Sprite::new(loader.get_resource_id("kotbulla"));
        sprite2.set_position(-300.0, -0.0);

        let mut text = Text::new(loader.get_resource_id("jetbrains"), "Hello");
        text.position.x = 200.0;
        text.position.y = 400.0;

        world.spawn().insert(sprite).insert(Player());
        world.spawn().insert(sprite2);
        world.spawn().insert(text);
    }
}
