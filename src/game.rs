
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Play,
    Lose,
    Win,
}

pub fn should_update() -> bool {
    use current_game_state;
    
    match unsafe { *current_game_state() } {
        GameState::Play => true,
        _ => false
    }
}

pub fn update_game_state() {
    use current_game_state;
    use current_blood_bar;
    // use current_win_music;
    // use current_lose_music;
    use blood_bar::BloodBar;

    let state = unsafe { &mut *current_game_state() };
    let &mut BloodBar(blood_bar) = unsafe { &mut *current_blood_bar() };

    *state = match *state {
        GameState::Play => {
            if won() {
                // unsafe { current_win_music() }.play();

                GameState::Win
            } else if blood_bar == 0.0 {
                // unsafe { current_lose_music() }.play();

                GameState::Lose
            } else {
                GameState::Play
            }
        }
        x => x,
    }
}

pub fn won() -> bool {
    use current_player;
    use settings::BEACH_ELLIPSE;

    let player = unsafe { &mut *current_player() };
    let (x, y) = (player.pos[0], player.pos[1]);
    let beach = BEACH_ELLIPSE;
    let rw = 0.5 * beach[2];
    let rh = 0.5 * beach[3];
    let cx = beach[0] + rw;
    let cy = beach[1] + rh;
    let dx = (x - cx) / rw;
    let dy = (y - cy) / rh;
    if dx * dx + dy * dy < 1.0 {
        true
    } else {
        false
    }
}

