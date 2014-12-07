
pub struct Player {
    pub pos: [f64, ..2],
    pub vel: [f64, ..2],
    pub acc: [f64, ..2],
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
    use settings::player::SPEEDUP;
    use std::num::Float;

    let dt = dt * SPEEDUP;

    let stream = &mut *current_stream();
    let player = &mut *current_player();
    let rocks = &mut *current_rocks();    
    let friction = WATER_FRICTION;

    let next_vel = add(player.vel, scale(player.acc, dt));
    let next_vel_square_len = square_len(next_vel);
    let drag = 1.0 / (next_vel_square_len * friction).exp();
    let next_vel = scale(next_vel, drag);

    // println!("TEST drag {}", drag);
 
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

pub fn move_left() {
    use current_player;
    use settings::player::ACC;    

    let player = &mut *current_player();

    player.acc = [-ACC, 0.0];
}

pub fn move_right() {
    use current_player;
    use settings::player::ACC;    

    let player = &mut *current_player();

    player.acc = [ACC, 0.0];
}

pub fn move_up() {
    use current_player;
    use settings::player::ACC;    

    let player = &mut *current_player();

    player.acc = [0.0, -ACC];
}

pub fn move_down() {
    use current_player;
    use settings::player::ACC;    

    let player = &mut *current_player();

    player.acc = [0.0, ACC];
}

