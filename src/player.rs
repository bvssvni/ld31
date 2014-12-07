
#![allow(dead_code)]

bitflags! {
    flags KeyState: u8 {
        const LEFT = 0b1,
        const RIGHT = 0b10,
        const UP = 0b100,
        const DOWN = 0b1000,
    }
}

pub struct Player {
    pub pos: [f64, ..2],
    pub vel: [f64, ..2],
    pub key_state: KeyState,
}

pub fn update_player(dt: f64) {
    use current_stream;
    use current_player;
    use current_rocks;
    use piston::vecmath::vec2_add as add;
    use piston::vecmath::vec2_scale as scale;
    use piston::vecmath::vec2_sub as sub;
    use piston::vecmath::vec2_len as len;
    use piston::vecmath::vec2_square_len as square_len;
    use settings::WATER_FRICTION;
    use settings::player::{ ACC, SPEEDUP };
    use std::num::Float;

    let dt = dt * SPEEDUP;

    let stream = &mut *current_stream();
    let player = &mut *current_player();
    let rocks = &mut *current_rocks();    
    let friction = WATER_FRICTION;

    let mut acc: [f64, ..2] = [0.0, 0.0];
    if player.key_state.contains(LEFT) {
        acc[0] -= ACC;
    }
    if player.key_state.contains(RIGHT) {
        acc[0] += ACC;
    }
    if player.key_state.contains(DOWN) {
        acc[1] += ACC;
    }
    if player.key_state.contains(UP) {
        acc[1] -= ACC;
    }

    let next_vel = add(player.vel, scale(acc, dt));
    let next_vel_square_len = square_len(next_vel);
    let drag = 1.0 / (next_vel_square_len * friction).exp();
    let next_vel = scale(next_vel, drag);

    let avg_vel = scale(add(player.vel, next_vel), 0.5);
    let dir = stream.at(player.pos);
    let next_pos = add(player.pos, add(scale(dir, dt), scale(avg_vel, dt)));

    let mut hits_rock = false;
    let rock_radius = ::settings::rocks::RADIUS;
    for rock in rocks.rocks.iter() {
        let diff = sub(rock.pos, next_pos);
        if len(diff) < rock_radius {
            hits_rock = true;
            break;
        }
    }

    if !hits_rock {
        player.pos = next_pos;
    }
}
