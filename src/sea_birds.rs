use piston::event::GenericEvent;
use piston::ai_behavior;

#[deriving(Clone)]
pub enum Action {
    /// Circles forever around target pos.
    Circling,
    /// Waits until player is within distance.
    PlayerWithinDistance(f64),
    /// Fly toward player.
    FlyTowardPlayer,
    /// Waits until player is far away from target.
    PlayerFarAwayFromTarget(f64),
    /// Makes player loose more blood.
    AttackPlayer(f64),
}

/// Sea birds fly around a target,
/// and if you get within a radius of the target it will attack you.
pub struct SeaBird {
    pub pos: [f64, ..2],
    pub dir: [f64, ..2],
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
            dir: [1.0, 0.0],
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
        use piston::ai_behavior::{ 
            While, Action, WaitForever, WhenAny, Wait, Sequence
        };

        let circling = Action(Action::Circling);
        let circle_until_player_within_distance =
            Sequence(vec![
                While(box Wait(5.0), vec![
                    circling.clone()
                ]),
                While(box Action(Action::PlayerWithinDistance(50.0)), vec![
                    circling.clone()
                ]),
            ]);
        let give_up_or_attack = WhenAny(vec![
            Action(Action::PlayerFarAwayFromTarget(100.0)),
            Sequence(vec![
                Action(Action::PlayerWithinDistance(10.0)),
                Action(Action::AttackPlayer(0.1)),
            ])
        ]);
        let attack_attempt =
            While(box give_up_or_attack, vec![
                Action(Action::FlyTowardPlayer)
            ]);
        let behavior = While(box WaitForever, vec![
            circle_until_player_within_distance,
            attack_attempt,
        ]);
        SeaBirds {
            birds: Vec::new(),
            behavior: behavior,
        }
    }
}

pub fn update_sea_birds<E: GenericEvent>(e: &E) {
    use current_sea_birds;
    use current_player;
    use piston::vecmath::vec2_add as add;
    use piston::vecmath::vec2_scale as scale;
    use piston::vecmath::vec2_sub as sub;
    use piston::vecmath::vec2_len as len;
    use piston::vecmath::vec2_normalized_sub as normalized_sub;
    use piston::vecmath::consts::Radians;
    use settings::sea_birds::circling;
    use settings::sea_birds::SPEEDUP;
    use std::num::FloatMath;
    use blood_bar;
    use player;

    let sea_birds = unsafe { &mut *current_sea_birds() };
    let player = unsafe { &mut *current_player() };

    let _360: f64 = Radians::_360();
    for sea_bird in sea_birds.birds.iter_mut() {
        let &SeaBird {
            ref mut state,
            ref mut circling_angle,
            ref target,
            ref mut pos,
            ref mut dir,
            ..
        } = sea_bird;
        state.event(e, |_, dt, action, _| {
             match *action {
                Action::Circling => {
                    let angle = *circling_angle;
                    let angle_pos = add(*target, 
                        scale([angle.cos(), angle.sin()], circling::RADIUS));
                    *dir = normalized_sub(angle_pos, *pos);
                    *pos = add(*pos, scale(*dir, dt * SPEEDUP * circling::SPEED));

                    let diff = sub(angle_pos, *pos);
                    let diff_len = len(diff);
                    if diff_len < circling::ADVANCE_RADIUS {
                        *circling_angle = angle + _360 / circling::N;
                    }
                    
                    (ai_behavior::Running, 0.0)
                }
                Action::PlayerWithinDistance(dist) => {
                    let diff = sub(*pos, player.pos);
                    if len(diff) < dist {
                        (ai_behavior::Success, dt)
                    } else {
                        (ai_behavior::Running, 0.0)
                    }
                }
                Action::PlayerFarAwayFromTarget(dist) => {
                    let diff = sub(*target, player.pos);
                    if len(diff) > dist {
                        (ai_behavior::Success, dt)
                    } else {
                        (ai_behavior::Running, 0.0)
                    }
                }
                Action::FlyTowardPlayer => {
                    *dir = normalized_sub(player.pos, *pos);
                    *pos = add(*pos, scale(*dir, dt * SPEEDUP * circling::SPEED));
                    (ai_behavior::Running, 0.0)
                }
                Action::AttackPlayer(val) => {
                    player.state = player::State::Bitten(::settings::player::BITTEN_FADE_OUT_SECONDS);
                    blood_bar::decrease(val);
                    (ai_behavior::Success, dt)                    
                }
            }
        });
    }
}
