use graphics::Context;
use opengl_graphics::{ Texture, GlGraphics };

pub struct BloodText(pub Texture);
pub struct YouWinText(pub Texture);
pub struct YouLoseText(pub Texture);
pub struct PalmTree(pub Texture);
pub struct SeaBird(pub Texture);
pub struct Rock(pub Texture);
pub struct Character(pub Texture);

pub fn render(c: &Context, g: &mut GlGraphics) {
    use settings::EDIT;

    if EDIT { stream_arrows(c, g); }
    moving_arrows(c, g);
    blood(c, g);
    beach(c, g);
    rocks(c, g);
    palm_tree(c, g);
    player(c, g);
    sea_birds(c, g);
    blood_bar(c, g);
    you_win(c, g);
    you_lose(c, g);
}

pub fn stream_arrows(c: &Context, g: &mut GlGraphics) {
    use current_stream;
    use graphics;
    use settings::stream::{ ARROW_COLOR, ARROW_SIZE, SPEEDUP };

    let stream = unsafe { &mut *current_stream() };
    let line = graphics::Line::new(ARROW_COLOR, 2.0);
    for arrow in stream.arrows.iter() {
        line.draw_arrow(arrow.line(SPEEDUP), ARROW_SIZE,
            &c.draw_state, c.transform, g);
    }
}

pub fn moving_arrows(c: &Context, g: &mut GlGraphics) {
    use current_moving_arrows;
    use graphics::Line;
    use settings::stream::{
        MOVING_ARROW_COLOR, 
        MOVING_ARROW_TIME_SPAN,
        SPEEDUP,
    };

    let moving_arrows = unsafe { &mut *current_moving_arrows() };
    let mut line = Line::new(MOVING_ARROW_COLOR, 1.0);
    for moving_arrow in moving_arrows.iter() {
        let line_color = line.color;
        let red = line_color[0];
        let green = line_color[1];
        let blue = line_color[2];
        let f = 2.0 * (moving_arrow.time / MOVING_ARROW_TIME_SPAN - 0.5);
        let alpha = 1.0 - f * f;
        line.color = [red, green, blue, alpha as f32];
        // line.draw_arrow(moving_arrow.arrow.line(SPEEDUP), ARROW_SIZE, c, g);
        line.draw(moving_arrow.arrow.line(SPEEDUP),
            &c.draw_state, c.transform, g);
    }
}

pub fn beach(c: &Context, g: &mut GlGraphics) {
    use graphics::Ellipse;
    use settings::{ beach_color, BEACH_ELLIPSE };

    Ellipse::new(beach_color()).draw(BEACH_ELLIPSE,
        &c.draw_state, c.transform, g);
}

pub fn player(c: &Context, g: &mut GlGraphics) {
    use current_player;
    use current_character;
    use graphics::{ Image, Transformed };
    use interpolation::lerp;
    use player::State;
    use settings::player::{ 
        FRAMES, BITTEN_COLOR, BITTEN_FADE_OUT_SECONDS,
    };

    let &mut Character(ref texture ) = unsafe { &mut *current_character() };
    let player = unsafe { &mut *current_player() };
    let (x, y) = (player.pos[0], player.pos[1]);
    // Rectangle::new(TEST_COLOR).draw(centered_square(x, y, radius), c, g);
    let frame = FRAMES[player.frame];
    match player.state {
        State::Bitten(sec) => {
            let t = 1.0 - sec / BITTEN_FADE_OUT_SECONDS;
            let color = lerp(&BITTEN_COLOR, &[1.0; 4], &(t as f32));
            Image::new_colored(color)
        }
        _ => Image::new()
    }.src_rect(frame).draw(texture, 
        &c.draw_state,
        c.transform.trans(x, y)
          .zoom(2.0)
          .trans(-0.5 * frame[2] as f64, -0.5 * frame[3] as f64), 
        g
    );
}

pub fn rocks(c: &Context, g: &mut GlGraphics) {
    use current_rocks;
    use current_rock;
    use graphics::image;
    use graphics::Transformed;
    use graphics::ImageSize;

    let &mut Rock(ref texture) = unsafe { &mut *current_rock() };
    let (w, h) = texture.get_size();
    let (w, h) = (w as f64, h as f64);
    let rocks = unsafe { &mut *current_rocks() };
    // let ellipse = Ellipse::new(TEST_COLOR);
    for rock in rocks.rocks.iter() {
        let (x, y) = (rock.pos[0], rock.pos[1]);
        // ellipse.draw(circle(x, y, radius), c, g);
        image(texture, c.transform.trans(x, y).zoom(1.05).trans(-0.5 * w, -0.5 * h), g);
    }
}

pub fn blood_bar(c: &Context, g: &mut GlGraphics) {
    use current_blood_text;
    use current_blood_bar;
    use graphics::image;
    use graphics::Transformed;
    use graphics::Rectangle;
    use graphics::rectangle::margin;
    use settings::blood_bar::{
        TEXT_POS, ZOOM, BAR_POS, BAR_SIZE,
        ROUND_RADIUS, background_color,
        foreground_color, MARGIN
    };
    use blood_bar::BloodBar;

    let &mut BloodText(ref blood_text) = unsafe { &mut *current_blood_text() };
    let &mut BloodBar(bar) = unsafe { &mut *current_blood_bar() };
    
    let pos = TEXT_POS;
    let zoom = ZOOM;
    image(blood_text, c.transform.trans(pos[0], pos[1]).zoom(zoom), g);

    let rect = [BAR_POS[0], BAR_POS[1] - BAR_SIZE[1], BAR_SIZE[0], BAR_SIZE[1]];
    Rectangle::new_round(background_color(), ROUND_RADIUS).draw(rect, &c.draw_state, c.transform, g);    
    let full_bar_height = BAR_SIZE[1] - 2.0 * MARGIN;
    let bar_height = full_bar_height * bar;
    let rect = [
        BAR_POS[0] + MARGIN, 
        BAR_POS[1] - BAR_SIZE[1] + MARGIN + (full_bar_height - bar_height), 
        BAR_SIZE[0] - 2.0 * MARGIN,
        bar_height
    ];
    Rectangle::new_round(foreground_color(), ROUND_RADIUS).draw(margin(rect, MARGIN),
        &c.draw_state, c.transform, g);
}

pub fn you_win(c: &Context, g: &mut GlGraphics) {
    use current_game_state;
    use current_you_win_text;
    use game::GameState;
    use graphics::image;
    use graphics::Transformed;
    use settings::you_win::{ POS, ZOOM };

    let game_state = unsafe { *current_game_state() };
    if game_state != GameState::Win { return; }

    let &mut YouWinText(ref texture) = unsafe { &mut *current_you_win_text() };
    image(texture, c.transform.trans(POS[0], POS[1]).zoom(ZOOM), g);
}

pub fn you_lose(c: &Context, g: &mut GlGraphics) {
    use current_game_state;
    use current_you_lose_text;
    use game::GameState;
    use graphics::image;
    use graphics::Transformed;
    use settings::you_lose::{ POS, ZOOM };

    let game_state = unsafe { *current_game_state() };
    if game_state != GameState::Lose { return; }

    let &mut YouLoseText(ref texture) = unsafe { &mut *current_you_lose_text() };
    image(texture, c.transform.trans(POS[0], POS[1]).zoom(ZOOM), g);
}

pub fn blood(c: &Context, g: &mut GlGraphics) {
    use current_blood;
    use graphics::Ellipse;
    use graphics::ellipse::circle;
    use settings::blood::{ test_color, RADIUS, SPAN, START_RADIUS };

    let blood = unsafe { &mut *current_blood() };

    let color = test_color();
    let red = color[0];
    let green = color[1];
    let blue = color[2];
    for blood_drop in blood.blood_drops.iter().filter(|e| !e.dead) {
        let (x, y) = (blood_drop.pos[0], blood_drop.pos[1]);
        let f = blood_drop.time / SPAN;
        let radius = START_RADIUS + (RADIUS - START_RADIUS) * f;
        let alpha = 1.0 - f;
        Ellipse::new([red, green, blue, alpha as f32]).draw(circle(x, y, radius),
            &c.draw_state, c.transform, g);
    }
}

pub fn palm_tree(c: &Context, g: &mut GlGraphics) {
    use current_palm_tree;
    use current_palm_trees;
    use graphics::image;
    use graphics::Transformed;

    let &mut PalmTree(ref texture) = unsafe { &mut *current_palm_tree() };
    let palm_trees = unsafe { &mut *current_palm_trees() };

    for pos in palm_trees.palms.iter() {
        image(texture, c.transform.trans(pos[0], pos[1]), g);
    }
}

pub fn sea_birds(c: &Context, g: &mut GlGraphics) {
    use current_sea_birds;
    use current_sea_bird;
    use graphics::image;
    use graphics::Transformed;

    let sea_birds = unsafe { &mut *current_sea_birds() };
    let &mut SeaBird(ref texture) = unsafe { &mut *current_sea_bird() };

    // let rect = Rectangle::new(TEST_COLOR);
    for sea_bird in sea_birds.birds.iter() {
        let (x, y) = (sea_bird.pos[0], sea_bird.pos[1]);
        let (dx, dy) = (sea_bird.dir[0], sea_bird.dir[1]);
        // rect.draw(centered_square(x, y, RADIUS), c, g);
        image(texture, c.transform.trans(x, y).orient(dx, dy).zoom(2.0).trans(-5.0, -6.0), g);
    }
}

