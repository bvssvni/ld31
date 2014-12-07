use piston::event::GenericEvent;
use piston::ai_behavior;

#[deriving(Clone)]
pub enum Action {
    Circling,
}

/// Sea birds fly around a target,
/// and if you get within a radius of the target it will attack you.
pub struct SeaBird {
    pub pos: [f64, ..2],
    pub target: [f64, ..2],
    pub circling_angle: f64,
    pub state: ai_behavior::State<Action, ()>,
}

impl SeaBird {
    pub fn new(
        pos: [f64, ..2], 
        target: [f64, ..2], 
        behavior: ai_behavior::Behavior<Action>
    ) -> SeaBird {
        SeaBird {
            pos: pos,
            target: target,
            state: ai_behavior::State::new(behavior),
            circling_angle: 0.0,
        }
    }
}

pub struct SeaBirds {
    pub birds: Vec<SeaBird>,
    pub behavior: ai_behavior::Behavior<Action>,
}

impl SeaBirds {
    pub fn new() -> SeaBirds {
        use piston::ai_behavior::{ While, WaitForever, Action };

        let behavior = While(box WaitForever, vec![
            Action(Action::Circling)
        ]);
        SeaBirds {
            birds: Vec::new(),
            behavior: behavior,
        }
    }
}

pub fn update_sea_birds<E: GenericEvent>(e: &E) {
    use current_sea_birds;
    use piston::vecmath::vec2_add as add;
    use piston::vecmath::vec2_scale as scale;
    use piston::vecmath::vec2_sub as sub;
    use piston::vecmath::vec2_len as len;
    use piston::vecmath::vec2_normalized_sub as normalized_sub;
    use piston::vecmath::consts::Radians;
    use settings::sea_birds::circling;
    use settings::sea_birds::SPEEDUP;
    use std::num::FloatMath;

    let sea_birds = &mut *current_sea_birds();

    let _360: f64 = Radians::_360();
    for sea_bird in sea_birds.birds.iter_mut() {
        let &SeaBird {
            ref mut state,
            ref mut circling_angle,
            ref target,
            ref mut pos,
            ..
        } = sea_bird;
        state.event(e, |_, dt, action, _| {
             match *action {
                Action::Circling => {
                    let angle = *circling_angle;
                    let angle_pos = add(*target, 
                        scale([angle.cos(), angle.sin()], circling::RADIUS));
                    let dir = normalized_sub(angle_pos, *pos);
                    *pos = add(*pos, scale(dir, dt * SPEEDUP * circling::SPEED));

                    let diff = sub(angle_pos, *pos);
                    let diff_len = len(diff);
                    if diff_len < circling::ADVANCE_RADIUS {
                        *circling_angle = angle + _360 / circling::N;
                    }
                    
                    (ai_behavior::Running, 0.0)
                }
            }
        });
    }
}
