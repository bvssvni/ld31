#![feature(default_type_params)]

extern crate piston;
extern crate opengl_graphics;
extern crate serialize;
extern crate sdl2_mixer;
extern crate sdl2;

use piston::current::{ Current, CurrentGuard };
use sdl2_mixer as mix;

mod blood;
mod blood_bar;
mod game;
mod palm_trees;
mod player;
mod render;
mod rocks;
mod settings;
mod stream;
mod sea_birds;

fn main() {
    let opengl = piston::shader_version::OpenGL::_3_2;
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

fn init_audio() {
    sdl2::init(sdl2::INIT_AUDIO | sdl2::INIT_TIMER);
    // Load dynamic libraries.
    mix::init(
          mix::INIT_MP3 
        | mix::INIT_FLAC 
        | mix::INIT_MOD 
        | mix::INIT_FLUIDSYNTH
        | mix::INIT_MODPLUG
        | mix::INIT_OGG
    );
    mix::open_audio(
        mix::DEFAULT_FREQUENCY,
        mix::DEFAULT_FORMAT,
        mix::DEFAULT_CHANNELS,
        1024
    ).unwrap();
    mix::allocate_channels(mix::DEFAULT_CHANNELS); 
}

pub struct WinMusic(pub mix::Music);

impl WinMusic {
    pub fn play(&mut self) {
        let &WinMusic(ref mut music) = self;
        music.play(0).unwrap();
    }
}

pub struct LoseMusic(pub mix::Music);

impl LoseMusic {
    pub fn play(&mut self) {
        let &LoseMusic(ref mut music) = self;
        music.play(0).unwrap();
    }
}

#[cfg(feature = "ship")]
pub fn root() -> Path {
    std::os::self_exe_path().unwrap()
}

#[cfg(not(feature = "ship"))]
pub fn root() -> Path {
    Path::new("./")
}

fn load_assets(f: ||) {
    use opengl_graphics::Texture;

    init_audio();

    let root = root();

    // Load music file. 
    let background_music = root.join(Path::new("assets/background.wav"));
    let win_music = root.join(Path::new("./assets/win.wav"));
    let lose_music = root.join(Path::new("./assets/lose.wav"));
    
    let background_music = mix::Music::from_file(&background_music).unwrap();

    let blood = root.join(Path::new("./assets/blood.png"));
    let you_win = root.join(Path::new("./assets/you-win.png"));
    let you_lose = root.join(Path::new("./assets/you-lose.png"));
    let palm_tree = root.join(Path::new("./assets/palm-tree.png"));
    let sea_bird = root.join(Path::new("./assets/sea-bird.png"));
    let rock = root.join(Path::new("./assets/rock.png"));
    let character = root.join(Path::new("./assets/character.png"));
 
    let mut win_music = WinMusic(mix::Music::from_file(&win_music).unwrap());
    let mut lose_music = LoseMusic(mix::Music::from_file(&lose_music).unwrap());
    let mut blood_text = render::BloodText(Texture::from_path(&blood).unwrap());
    let mut you_win_text = render::YouWinText(Texture::from_path(&you_win).unwrap());
    let mut you_lose_text = render::YouLoseText(Texture::from_path(&you_lose).unwrap());
    let mut palm_tree = render::PalmTree(Texture::from_path(&palm_tree).unwrap());
    let mut sea_bird = render::SeaBird(Texture::from_path(&sea_bird).unwrap());
    let mut rock = render::Rock(Texture::from_path(&rock).unwrap());
    let mut character = render::Character(Texture::from_path(&character).unwrap());

    let blood_text_guard = CurrentGuard::new(&mut blood_text);
    let you_win_text_guard = CurrentGuard::new(&mut you_win_text);
    let you_lose_text_guard = CurrentGuard::new(&mut you_lose_text);
    let palm_tree_guard = CurrentGuard::new(&mut palm_tree);
    let sea_bird_guard = CurrentGuard::new(&mut sea_bird);
    let rock_guard = CurrentGuard::new(&mut rock);
    let character_guard = CurrentGuard::new(&mut character);
    let win_music_guard = CurrentGuard::new(&mut win_music);
    let lose_music_guard = CurrentGuard::new(&mut lose_music);

    // Restart level if not quiting.
    while !piston::should_close() {
        // Loop infinite times. 
        background_music.play(-1).unwrap();
        
        f();
    }

    drop(blood_text_guard);
    drop(you_win_text_guard);
    drop(you_lose_text_guard);
    drop(palm_tree_guard);
    drop(sea_bird_guard);
    drop(rock_guard);
    drop(character_guard);
    drop(win_music_guard);
    drop(lose_music_guard);
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
    let mut player = player::Player::new(settings::player::START_POS);
    let mut rocks = rocks::Rocks { rocks: Vec::new() };
    let mut selected_arrow = stream::SelectedArrow(None);
    let mut game_state = game::GameState::Play;
    let mut blood_bar = blood_bar::BloodBar(settings::blood_bar::START_VAL);
    let mut blood = blood::Blood {
        blood_drops: Vec::new(),
        time_since_last_drop: 0.0,
    };
    let mut palm_trees = palm_trees::PalmTrees { palms: Vec::new() };
    let mut sea_birds = sea_birds::SeaBirds::new();

    let stream_guard = CurrentGuard::new(&mut stream);
    let moving_arrows = CurrentGuard::new(&mut moving_arrows);
    let player_guard = CurrentGuard::new(&mut player);
    let rocks_guard = CurrentGuard::new(&mut rocks);
    let selected_arrow_guard = CurrentGuard::new(&mut selected_arrow);
    let game_state_guard = CurrentGuard::new(&mut game_state);
    let blood_bar_guard = CurrentGuard::new(&mut blood_bar);
    let blood_guard = CurrentGuard::new(&mut blood);
    let palm_trees_guard = CurrentGuard::new(&mut palm_trees);
    let sea_birds_guard = CurrentGuard::new(&mut sea_birds);

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
    drop(sea_birds_guard);
}

fn sea_rect() -> [f64, ..4] {
    use piston::current::Get;

    let piston::window::DrawSize([w, h]) = piston::current_window().get();
    [0.0, 0.0, w as f64, h as f64]
}

pub unsafe fn current_stream() -> Current<stream::Stream> { Current::new() }
pub unsafe fn current_moving_arrows() -> Current<Vec<stream::MovingArrow>> { Current::new() }
pub unsafe fn current_player() -> Current<player::Player> { Current::new() }
pub unsafe fn current_rocks() -> Current<rocks::Rocks> { Current::new() }
pub unsafe fn current_selected_arrow() -> Current<stream::SelectedArrow> { Current::new() }
pub unsafe fn current_game_state() -> Current<game::GameState> { Current::new() }
pub unsafe fn current_blood_text() -> Current<render::BloodText> { Current::new() }
pub unsafe fn current_blood_bar() -> Current<blood_bar::BloodBar> { Current::new() }
pub unsafe fn current_you_win_text() -> Current<render::YouWinText> { Current::new() }
pub unsafe fn current_you_lose_text() -> Current<render::YouLoseText> { Current::new() }
pub unsafe fn current_blood() -> Current<blood::Blood> { Current::new() }
pub unsafe fn current_palm_tree() -> Current<render::PalmTree> { Current::new() }
pub unsafe fn current_palm_trees() -> Current<palm_trees::PalmTrees> { Current::new() }
pub unsafe fn current_sea_birds() -> Current<sea_birds::SeaBirds> { Current::new() }
pub unsafe fn current_sea_bird() -> Current<render::SeaBird> { Current::new() }
pub unsafe fn current_rock() -> Current<render::Rock> { Current::new() }
pub unsafe fn current_character() -> Current<render::Character> { Current::new() }
pub unsafe fn current_win_music() -> Current<WinMusic> { Current::new() }
pub unsafe fn current_lose_music() -> Current<LoseMusic> { Current::new() }

fn start() {
    settings::stream::load();
    stream::refresh_moving_arrows();
    settings::rocks::load();
    settings::palm_trees::load();
    settings::sea_birds::load();

    let mut cursor: [f64, ..2] = [0.0, ..2];
    for e in piston::events() {
        use piston::event::{ 
            MouseCursorEvent, PressEvent, 
            ReleaseEvent, RenderEvent, UpdateEvent
        };
        let e: piston::event::Event<piston::input::Input> = e;
        e.render(|_args| {
            piston::render_2d_opengl(
                Some(settings::background_color()), |c, g| {
                render::render(&c, g);
            });

            piston::set_title(
                format!("Sea Birds' Breakfast ({})", piston::fps_tick())
            );
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
        if game::should_update() {
            sea_birds::update_sea_birds(&e);
        }

        e.mouse_cursor(|x, y| {
            cursor = [x, y];
            if settings::EDIT {
                stream::edit_selected_arrow(cursor);
            }
        });
        e.press(|button| {
            if settings::EDIT {
                if button == settings::stream::ADD_ARROW_BUTTON {
                    stream::add_arrow(cursor);
                    stream::refresh_moving_arrows();
                }
                if button == settings::utils::PRINT_CURSOR_POS {
                    println!("{}, {},", cursor[0], cursor[1]);
                }
                if button == settings::utils::PRINT_PLAYER_POS {
                    let pos = unsafe { current_player() }.pos;
                    println!("{}, {},", pos[0], pos[1]);
                }
                if button == settings::utils::PRINT_STREAM {
                    println!("Stream:");
                    for (arrow, phase) in unsafe { current_stream() }.arrows.iter().zip(
                        unsafe { current_stream() }.arrow_phases.iter()) {
                        println!("{}, {}, {}, {}, {},", arrow.pos[0], arrow.pos[1],
                            arrow.dir[0], arrow.dir[1], *phase);
                    }
                }
                if button == settings::utils::PRINT_HAS_WON {
                    println!("{}", game::won());
                }
            }
            if button == settings::player::MOVE_LEFT_BUTTON {
                unsafe { current_player() }.key_state.insert(player::LEFT);
            }
            if button == settings::player::MOVE_RIGHT_BUTTON {
                unsafe { current_player() }.key_state.insert(player::RIGHT);
            }
            if button == settings::player::MOVE_UP_BUTTON {
                unsafe { current_player() }.key_state.insert(player::UP);
            }
            if button == settings::player::MOVE_DOWN_BUTTON {
                unsafe { current_player() }.key_state.insert(player::DOWN);
            }
        });
        e.release(|button| {
            if button == settings::stream::ADD_ARROW_BUTTON {
                stream::deselect_arrow();
            }
            if button == settings::player::MOVE_LEFT_BUTTON {
                unsafe { current_player() }.key_state.remove(player::LEFT);
            }
            if button == settings::player::MOVE_RIGHT_BUTTON {
                unsafe { current_player() }.key_state.remove(player::RIGHT);
            }
            if button == settings::player::MOVE_UP_BUTTON {
                unsafe { current_player() }.key_state.remove(player::UP);
            }
            if button == settings::player::MOVE_DOWN_BUTTON {
                unsafe { current_player() }.key_state.remove(player::DOWN);
            }
        });

        let restart = e.press(|button| {
            let game_state = unsafe { *current_game_state() };
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

*/

