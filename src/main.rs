extern crate piston;
extern crate opengl_graphics;
extern crate serialize;

use piston::current::{ Current, CurrentGuard };

mod blood;
mod blood_bar;
mod game;
mod palm_trees;
mod player;
mod render;
mod rocks;
mod settings;
mod stream;

fn main() {
    let opengl = piston::shader_version::opengl::OpenGL::OpenGL_3_2;
    piston::start(
        opengl,
        piston::WindowSettings {
            title: "Sea Birds' Breakfast".to_string(),
            size: [640, 480],
            samples: 4,
            fullscreen: false,
            exit_on_esc: true,
        },
        || load_assets(|| setup())
    );
}

fn load_assets(f: ||) {
    use opengl_graphics::Texture;

    let blood = Path::new("./assets/blood.png");
    let you_win = Path::new("./assets/you-win.png");
    let you_lose = Path::new("./assets/you-lose.png");
    let palm_tree = Path::new("./assets/palm-tree.png");
    
    let mut blood_text = render::BloodText(Texture::from_path(&blood).unwrap());
    let mut you_win_text = render::YouWinText(Texture::from_path(&you_win).unwrap());
    let mut you_lose_text = render::YouLoseText(Texture::from_path(&you_lose).unwrap());
    let mut palm_tree = render::PalmTree(Texture::from_path(&palm_tree).unwrap());

    let blood_text_guard = CurrentGuard::new(&mut blood_text);
    let you_win_text_guard = CurrentGuard::new(&mut you_win_text);
    let you_lose_text_guard = CurrentGuard::new(&mut you_lose_text);
    let palm_tree_guard = CurrentGuard::new(&mut palm_tree);

    // Restart level if not quiting.
    while !piston::should_close() {
        f();
    }

    drop(blood_text_guard);
    drop(you_win_text_guard);
    drop(you_lose_text_guard);
    drop(palm_tree_guard);
}

/// Initialize current objects used as application structure
fn setup() {
    let mut stream = stream::Stream {
        arrows: Vec::new(),
        rect: sea_rect(),
        strength: settings::stream::STRENGTH,
        arrow_phases: Vec::new(),
    };
    let mut moving_arrows: Vec<stream::MovingArrow> = Vec::new();    
    let mut player = player::Player {
            pos: settings::player::START_POS,
            vel: settings::player::START_VEL,
            acc: [0.0, ..2],
        };
    let mut rocks = rocks::Rocks { rocks: Vec::new() };
    let mut selected_arrow = stream::SelectedArrow(None);
    let mut game_state = game::GameState::Play;
    let mut blood_bar = blood_bar::BloodBar(settings::blood_bar::START_VAL);
    let mut blood = blood::Blood {
        blood_drops: Vec::new(),
        time_since_last_drop: 0.0,
    };
    let mut palm_trees = palm_trees::PalmTrees { palms: Vec::new() };

    let stream_guard = CurrentGuard::new(&mut stream);
    let moving_arrows = CurrentGuard::new(&mut moving_arrows);
    let player_guard = CurrentGuard::new(&mut player);
    let rocks_guard = CurrentGuard::new(&mut rocks);
    let selected_arrow_guard = CurrentGuard::new(&mut selected_arrow);
    let game_state_guard = CurrentGuard::new(&mut game_state);
    let blood_bar_guard = CurrentGuard::new(&mut blood_bar);
    let blood_guard = CurrentGuard::new(&mut blood);
    let palm_trees_guard = CurrentGuard::new(&mut palm_trees);

    start();
    
    drop(stream_guard);
    drop(moving_arrows);
    drop(player_guard);
    drop(rocks_guard);
    drop(selected_arrow_guard);
    drop(game_state_guard);
    drop(blood_bar_guard);
    drop(blood_guard);
    drop(palm_trees_guard);
}

fn sea_rect() -> [f64, ..4] {
    use piston::current::Get;

    let piston::window::DrawSize([w, h]) = unsafe { piston::current_window().get() };
    [0.0, 0.0, w as f64, h as f64]
}

pub fn current_stream() -> Current<stream::Stream> { Current }
pub fn current_moving_arrows() -> Current<Vec<stream::MovingArrow>> { Current }
pub fn current_player() -> Current<player::Player> { Current }
pub fn current_rocks() -> Current<rocks::Rocks> { Current }
pub fn current_selected_arrow() -> Current<stream::SelectedArrow> { Current }
pub fn current_game_state() -> Current<game::GameState> { Current }
pub fn current_blood_text() -> Current<render::BloodText> { Current }
pub fn current_blood_bar() -> Current<blood_bar::BloodBar> { Current }
pub fn current_you_win_text() -> Current<render::YouWinText> { Current }
pub fn current_you_lose_text() -> Current<render::YouLoseText> { Current }
pub fn current_blood() -> Current<blood::Blood> { Current }
pub fn current_palm_tree() -> Current<render::PalmTree> { Current }
pub fn current_palm_trees() -> Current<palm_trees::PalmTrees> { Current }

fn start() {
    stream::refresh_moving_arrows();
    settings::rocks::load();
    settings::palm_trees::load();

    let mut cursor: [f64, ..2] = [0.0, ..2];
    for e in piston::events() {
        use piston::event::{ 
            MouseCursorEvent, PressEvent, 
            ReleaseEvent, RenderEvent, UpdateEvent
        };
        e.render(|_args| {
            piston::render_2d_opengl(Some(settings::background_color()), |c, g| {
                render::render(&c, g);
            });

            piston::set_title(piston::fps_tick().to_string());
        });
        e.update(|args| {
            let dt = args.dt;
            if game::should_update() {
                stream::update_stream(dt);
                stream::update_moving_arrows(dt);
                player::update_player(dt);
                blood_bar::update_blood_bar(dt);
                blood::update_blood(dt);
            }

            game::update_game_state();
        });
        e.mouse_cursor(|x, y| {
            cursor = [x, y];
            stream::edit_selected_arrow(cursor);
        });
        e.press(|button| {
            if button == settings::stream::ADD_ARROW_BUTTON {
                stream::add_arrow(cursor);
                stream::refresh_moving_arrows();
            }
            if button == settings::utils::PRINT_CURSOR_POS {
                println!("{}, {}", cursor[0], cursor[1]);
            }
            if button == settings::utils::PRINT_PLAYER_POS {
                let pos = current_player().pos;
                println!("{}, {}", pos[0], pos[1]);
            }

            if button == settings::utils::PRINT_HAS_WON {
                println!("{}", game::won());
            }
            if button == settings::player::MOVE_LEFT_BUTTON {
                player::move_left();
            }
            if button == settings::player::MOVE_RIGHT_BUTTON {
                player::move_right();
            }
            if button == settings::player::MOVE_UP_BUTTON {
                player::move_up();
            }
            if button == settings::player::MOVE_DOWN_BUTTON {
                player::move_down();
            }
        });
        e.release(|button| {
            if button == settings::stream::ADD_ARROW_BUTTON {
                stream::deselect_arrow();
            }
        });

        let restart = e.press(|button| {
            let game_state = *current_game_state();
            let can_restart = match game_state {
                    game::GameState::Win
                  | game::GameState::Lose => true,
                    _ => false
                };
            if can_restart && button == settings::utils::RESTART_LEVEL {
                true
            } else {
                false
            }
        });
        if restart == Some(true) { break; } 
    }
}

/*

- Add character texture
- Add sea birds
- See how the sea looks with ellipses instead of lines
- Add rock texture
- Make it possible to toggle on/off the arrows

I want the player to feel how it is to struggle with the current.

*/
