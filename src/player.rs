
#![allow(dead_code)]

bitflags! {
    flags KeyState: u8 {
        const LEFT = 0b1,
        const RIGHT = 0b10,
        const UP = 0b100,
        const DOWN = 0b1000,
    }
}

impl KeyState {
    pub fn acceleration(&self, d: f64) -> [f64; 2] {
        let mut acc = [0.0; 2];
        if self.contains(LEFT) {
            acc[0] -= d;
        }
        if self.contains(RIGHT) {
            acc[0] += d;
        }
        if self.contains(DOWN) {
            acc[1] += d;
        }
        if self.contains(UP) {
            acc[1] -= d;
        }
        acc
    }
}

pub enum State {
    Bitten(f64),
    Normal,
}

pub struct Player {
    pub pos: [f64; 2],
    pub vel: [f64; 2],
    pub key_state: KeyState,
    pub time_since_last_frame_update: f64,
    pub frame: usize,
    pub state: State,
}

impl Player {
    pub fn new(pos: [f64; 2]) -> Player {
        Player {
            pos: pos,
            vel: [0.0, 0.0],
            key_state: KeyState::empty(),
            time_since_last_frame_update: 0.0,
            frame: 0,
            state: State::Normal,
        }
    }
}

pub fn update_player(dt: f64) {
    use current_stream;
    use current_player;
    use current_rocks;
    use vecmath::vec2_add as add;
    use vecmath::vec2_scale as scale;
    use vecmath::vec2_sub as sub;
    use vecmath::vec2_len as len;
    use vecmath::vec2_square_len as square_len;
    use settings::WATER_FRICTION;
    use settings::player::{ ACC, FRAME_INTERVAL, FRAMES, SPEEDUP };

    let dt = dt * SPEEDUP;

    let stream = unsafe { &mut *current_stream() };
    let player = unsafe { &mut *current_player() };
    let rocks = unsafe { &mut *current_rocks() };    
    let friction = WATER_FRICTION;

    player.time_since_last_frame_update += dt;
    if player.time_since_last_frame_update > FRAME_INTERVAL {
        if player.key_state.contains(UP) {
            player.frame = (player.frame + FRAMES.len() - 1) % FRAMES.len();
        } else {
            player.frame = (player.frame + 1) % FRAMES.len();
        }
        player.time_since_last_frame_update -= FRAME_INTERVAL;
    }
    player.state = match player.state {
            State::Normal => State::Normal,
            State::Bitten(sec) => {
                let new_sec = sec - dt;
                if new_sec < 0.0 {
                    State::Normal
                } else {
                    State::Bitten(new_sec)
                }
            }
        };

    let acc = player.key_state.acceleration(ACC);

    let drag = (-square_len(player.vel) * friction).exp();
    let vel = scale(player.vel, drag);

    let next_vel = add(player.vel, scale(acc, dt));
    let drag = (-square_len(next_vel) * friction).exp();
    let next_vel = scale(next_vel, drag);

    player.vel = scale(add(vel, next_vel), 0.5);

    let dir = stream.at(player.pos);
    let next_pos = add(player.pos, add(scale(dir, dt), scale(player.vel, dt)));

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
