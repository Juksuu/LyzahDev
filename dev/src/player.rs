use legion::*;
use lyzah::{
    input::{Input, VirtualKeyCode},
    Sprite, Time,
};

pub struct Player();

#[system(par_for_each)]
pub fn move_player(
    sprite: &mut Sprite,
    _player: &Player,
    #[resource] time: &Time,
    #[resource] input: &Input,
) {
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
