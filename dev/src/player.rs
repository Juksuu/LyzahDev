use crate::game::Game;
use lyzah::{
    ecs as bevy_ecs,
    input::{Input, VirtualKeyCode},
    Sprite, Time,
};

use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Player();

pub fn move_player(
    mut query: Query<&mut Sprite, With<Player>>,
    time: Res<Time>,
    input: Res<Input>,
    game: Res<Game>,
) {
    if !game.is_running {
        return;
    }

    for mut sprite in query.iter_mut() {
        if input.is_mouse_inside_window {
            sprite.set_position(input.mouse_pos.x, input.mouse_pos.y);
        } else {
            let old_pos = sprite.position;
            let move_amount = 200.0 * time.delta_time.as_secs_f32();
            let mut pos_change = (0.0, 0.0);

            for key in &input.pressed_keys {
                match key {
                    VirtualKeyCode::A => pos_change.0 -= move_amount,
                    VirtualKeyCode::S => pos_change.1 -= move_amount,
                    VirtualKeyCode::W => pos_change.1 += move_amount,
                    VirtualKeyCode::D => pos_change.0 += move_amount,
                    _ => (),
                }
            }
            sprite.set_position(old_pos.x + pos_change.0, old_pos.y + pos_change.1);
        }
    }
}
