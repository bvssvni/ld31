
#[deriving(Copy)]
pub struct BloodDrop {
    pub pos: [f64, ..2],
    pub time: f64,
    pub dead: bool,
}

pub struct Blood {
    pub blood_drops: Vec<BloodDrop>,
    pub time_since_last_drop: f64,
}

pub fn update_blood(dt: f64) {
    use current_blood;
    use current_player;
    use current_stream;
    use piston::vecmath::vec2_add as add;
    use piston::vecmath::vec2_scale as scale;
    use settings::blood::{ DROP_INTERVAL, SPAN };

    let blood = &mut *current_blood();
    let player = &mut *current_player();
    let stream = &mut *current_stream();

    let interval = DROP_INTERVAL;
    blood.time_since_last_drop += dt;
    if blood.time_since_last_drop > interval {
        blood.time_since_last_drop -= interval;
        let new_drop = BloodDrop {
            pos: player.pos,
            time: 0.0,
            dead: false,
        };
        // Look for dead blood drops before inserting new.
        let mut found_dead = false;
        for blood_drop in blood.blood_drops.iter_mut() {
            if blood_drop.dead {
                *blood_drop = new_drop;
                found_dead = true;
                break;
            }
        }
        if !found_dead { blood.blood_drops.push(new_drop) };
    }

    // Make blood drops follow stream
    for blood_drop in blood.blood_drops.iter_mut() {
        blood_drop.pos = add(blood_drop.pos, scale(stream.at(blood_drop.pos), dt));
        blood_drop.time += dt;
        if blood_drop.time > SPAN {
            blood_drop.dead = true;
        }
    }
}

