use piston::event::GenericEvent;
use ai_behavior;

#[derive(Clone)]
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
    pub pos: [f64; 2],
    pub dir: [f64; 2],
    pub target: [f64; 2],
    pub circling_angle: f64,
    pub state: ai_behavior::State<Action, ()>,
}

impl SeaBird {
    pub fn new(
        pos: [f64; 2], 
        target: [f64; 2], 
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
        use ai_behavior::{ 
            While, Action, WaitForever, WhenAny, Wait, Sequence
        };

        let circling = Action(Action::Circling);
        let circle_until_player_within_distance =
            Sequence(vec![
                While(Box::new(Wait(5.0)), vec![
                    circling.clone()
                ]),
                While(Box::new(Action(Action::PlayerWithinDistance(50.0))), vec![
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
            While(Box::new(give_up_or_attack), vec![
                Action(Action::FlyTowardPlayer)
            ]);
        let behavior = While(Box::new(WaitForever), vec![
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
    use vecmath::vec2_add as add;
    use vecmath::vec2_scale as scale;
    use vecmath::vec2_sub as sub;
    use vecmath::vec2_len as len;
    use vecmath::vec2_normalized_sub as normalized_sub;
    use vecmath::traits::Radians;
    use settings::sea_birds::circling;
    use settings::sea_birds::SPEEDUP;
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
