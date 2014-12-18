use piston::graphics::color::hex;

pub const BEACH_ELLIPSE: [f64, ..4] = [500.0, -500.0, 1000.0, 1500.0];
pub const WATER_FRICTION: f64 = 0.0004;
pub const EDIT: bool = false;

pub fn background_color() -> [f32, ..4] {
    hex("49B1DE")
}

pub fn beach_color() -> [f32, ..4] {
    hex("F2EABD")
}

pub mod utils {
    use piston::input::{ Button };
    use piston::input::keyboard::Key;

    pub const PRINT_CURSOR_POS: Button = Button::Keyboard(Key::C);
    pub const PRINT_PLAYER_POS: Button = Button::Keyboard(Key::P);
    pub const PRINT_HAS_WON: Button = Button::Keyboard(Key::W);
    pub const PRINT_STREAM: Button = Button::Keyboard(Key::S);
    pub const RESTART_LEVEL: Button = Button::Keyboard(Key::Return);
}

pub mod you_win {
    pub const POS: [f64, ..2] = [193.0, 182.0];
    pub const ZOOM: f64 = 10.0;
}

pub mod you_lose {
    pub const POS: [f64, ..2] = [164.0, 198.0];
    pub const ZOOM: f64 = 10.0;
}


pub mod player {
    use piston::input::{ Button };
    use piston::input::keyboard::Key;

    pub const SPEEDUP: f64 = 2.0;
    // pub const RADIUS: f64 = 5.0;
    pub const START_POS: [f64, ..2] = [100.0, 100.0];
    // pub const START_VEL: [f64, ..2] = [0.0, 0.0];
    // pub const TEST_COLOR: [f32, ..4] = [1.0, ..4];
    pub const BITTEN_COLOR: [f32, ..4] = [1.0, 0.0, 0.0, 1.0];
    pub const BITTEN_FADE_OUT_SECONDS: f64 = 2.0;
    pub static FRAMES: &'static [[i32, ..4]] = &[
        [0, 0, 16, 16],
        [16, 0, 16, 16],
        [32, 0, 16, 16],
        [48, 0, 16, 16],
        [0, 16, 16, 16],
        [16, 16, 16, 16],
        [32, 16, 16, 16],
        [48, 16, 16, 16]
    ];
    pub static FRAME_INTERVAL: f64 = 0.2;

    pub const MOVE_LEFT_BUTTON: Button = Button::Keyboard(Key::Left);
    pub const MOVE_RIGHT_BUTTON: Button = Button::Keyboard(Key::Right);
    pub const MOVE_DOWN_BUTTON: Button = Button::Keyboard(Key::Down);
    pub const MOVE_UP_BUTTON: Button = Button::Keyboard(Key::Up);

    pub const ACC: f64 = 50.0;
}

pub mod stream {
    use piston::input::{ Button, MouseButton };
    
    pub const ADD_ARROW_BUTTON: Button = Button::Mouse(MouseButton::Left);
    pub const STRENGTH: [f64, ..2] = [10.0, 50.0];
    pub const ARROW_COLOR: [f32, ..4] = [0.0, 0.0, 0.7, 0.8];
    pub const ARROW_SIZE: f64 = 5.0;
    pub const SAMPLE_SIZE: f64 = 25.0;
    // pub const SAMPLE_COLOR: [f32, ..4] = [0.5, 0.5, 0.0, 1.0];
    pub const MOVING_ARROW_COLOR: [f32, ..4] = [1.0, 1.0, 0.8, 1.0];
    pub const MOVING_ARROW_TIME_SPAN: f64 = 40.0;
    pub const SPEEDUP: f64 = 1.0;
    pub const PHASE_VEL: f64 = 1.0 / 7.0;
    
    pub fn load() {
        use current_stream;
        use stream::Arrow;

        let stream = &mut *current_stream();

        // Just split by comma.
        let data = include_str!("../assets/stream.txt");
        let mut data_split = data.split_str(",");
        loop {
            let x: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            let y: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            let dx: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            let dy: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            let phase: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            stream.add_arrow(Arrow { pos: [x, y], dir: [dx, dy] }, phase);            
        }
    }
}

pub mod rocks {
    // pub const TEST_COLOR: [f32, ..4] = [0.6, 0.6, 0.6, 1.0];
    pub const RADIUS: f64 = 20.0;

    pub fn load() {
        use current_rocks;
        use rocks::Rock;

        let rocks = &mut *current_rocks();

        // Just split by comma.
        let data = include_str!("../assets/rocks.txt");
        let mut data_split = data.split_str(",");
        loop {
            let x: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            let y: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            rocks.rocks.push(Rock { pos: [x, y] });            
        }
    }
}

pub mod blood_bar {
    pub const TEXT_POS: [f64, ..2] = [567.0, 441.0];
    pub const ZOOM: f64 = 2.0;
    pub const START_VAL: f64 = 1.0;
    pub const BAR_POS: [f64, ..2] = [581.0, 433.0];
    pub const BAR_SIZE: [f64, ..2] = [10.0, 200.0];
    pub const ROUND_RADIUS: f64 = 5.0;
    pub const MARGIN: f64 = 1.0;
    pub const DEC_VAL: f64 = 0.005;
    // pub const DEC_VAL: f64 = 0.1;    

    pub fn background_color() -> [f32, ..4] {
        use piston::graphics::color::hex;

        hex("7D3E5F")
    }

    pub fn foreground_color() -> [f32, ..4] {
        use piston::graphics::color::hex;

        hex("FF0000")
    }
}

pub mod blood {
    pub const DROP_INTERVAL: f64 = 0.2;
    pub const START_RADIUS: f64 = 4.0;
    pub const RADIUS: f64 = 10.0;
    pub const SPAN: f64 = 50.0;

    pub fn test_color() -> [f32, ..4] {
        use piston::graphics::color::hex;

        hex("D12219")
    }
}

pub mod palm_trees {
    pub fn load() {
        use current_palm_trees;

        let palm_trees = &mut *current_palm_trees();

        // Just split by comma.
        let data = include_str!("../assets/palm_trees.txt");
        let mut data_split = data.split_str(",");
        loop {
            let x: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            let y: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            palm_trees.palms.push([x, y]);            
        }
    }
}

pub mod sea_birds {
    // pub const RADIUS: f64 = 5.0;
    // pub const TEST_COLOR: [f32, ..4] = [1.0, 1.0, 0.0, 1.0];
    pub const SPEEDUP: f64 = 5.0;

    pub mod circling {
        // How many segments to split up circling.
        pub const N: f64 = 128.0;
        // Must be within 5 pixels of target to go to next.
        pub const ADVANCE_RADIUS: f64 = 5.0;
        pub const SPEED: f64 = 4.0;
        pub const RADIUS: f64 = 50.0;
    }
 
    pub fn load() {
        use current_sea_birds;
        use sea_birds::SeaBird;

        let sea_birds = &mut *current_sea_birds();

        // Just split by comma.
        let data = include_str!("../assets/sea_birds.txt");
        let mut data_split = data.split_str(",");
        loop {
            let x: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            let y: f64 = match data_split.next() {
                    None => { break; }
                    Some(x) => from_str(x.trim()).unwrap()
                };
            sea_birds.birds.push(SeaBird::new(
                    [x, y],
                    [x, y],
                    sea_birds.behavior.clone(),
                ));
        }
    }
}

