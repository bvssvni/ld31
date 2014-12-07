//! Describes the sea current

/// The current selected arrow.
pub struct SelectedArrow(pub Option<uint>);

pub struct Arrow {
    pub pos: [f64, ..2],
    pub dir: [f64, ..2],
}

impl Arrow {
    pub fn line(&self, speedup: f64) -> [f64, ..4] {
        [
            self.pos[0],
            self.pos[1],
            self.pos[0] + self.dir[0] * speedup, 
            self.pos[1] + self.dir[1] * speedup
        ]
    }
}

pub struct MovingArrow {
    pub arrow: Arrow,
    pub start_pos: [f64, ..2],
    pub time: f64,
}

pub struct Stream {
    /// Arrows that describe the stream current
    pub arrows: Vec<Arrow>,
    /// A rectangle used to pick random coordinates
    pub rect: [f64, ..4],
    /// A range to pick random sea current strength
    pub strength: [f64, ..2],
    /// The phase of the arrow the moment they were added.
    pub arrow_phases: Vec<f64>,
}

impl Stream {
    /// Computes stream at position using weighted average
    pub fn at(&self, pos: [f64, ..2]) -> [f64, ..2] {
        use piston::vecmath::vec2_add as add;
        use piston::vecmath::vec2_sub as sub;
        use piston::vecmath::vec2_len as len;
        use piston::vecmath::vec2_scale as scale;
        use std::num::FloatMath;

        if self.arrows.len() == 0 { return [0.0, ..2]; }

        let mut sum = [0.0, ..2];
        let mut sum_w = 0.0;
        for (i, arrow) in self.arrows.iter().enumerate() {
            let l = len(sub(arrow.pos, pos)).max(0.01);
            let w = 1.0 / (l * l);
            let phase_w = self.arrow_phases[i].cos();
            sum = add(sum, scale(arrow.dir, w * phase_w));
            sum_w += w;
        }

        scale(sum, 1.0 / sum_w)
    }

    pub fn nm(&self, sample_size: f64) -> [u32, ..2] {
        let [_, _, w, h] = self.rect;
        [(w / sample_size) as u32, (h / sample_size) as u32]
    }
}

pub fn add_arrow(pos: [f64, ..2]) {
    use current_stream;
    use current_selected_arrow;
    use std::rand::random;
    use piston::vecmath::consts::Radians;

    let stream = &mut *current_stream();
    let selected_arrow = &mut *current_selected_arrow();

    stream.arrows.push(Arrow {
        pos: pos,
        dir: [0.0, ..2],
    });
    stream.arrow_phases.push(random::<f64>() * Radians::_360());

    let id = stream.arrows.len() - 1;
    *selected_arrow = SelectedArrow(Some(id));
}

pub fn edit_selected_arrow(pos: [f64, ..2]) {
    use current_stream;
    use current_selected_arrow;
    use piston::vecmath::vec2_sub as sub;
    use piston::vecmath::vec2_scale as scale;
    use settings::stream::SPEEDUP;

    let stream = &mut *current_stream();
    let &SelectedArrow(selected_arrow) = &mut *current_selected_arrow();
    let id = match selected_arrow {
        None => { return; }
        Some(x) => x
    };
    stream.arrows[id].dir = scale(sub(pos, stream.arrows[id].pos), 1.0 / SPEEDUP);
}

pub fn deselect_arrow() {
    use current_selected_arrow;

    *current_selected_arrow() = SelectedArrow(None);
}

pub fn refresh_moving_arrows() {
    use current_stream;
    use current_moving_arrows;
    use settings::stream::SAMPLE_SIZE;

    let stream = &mut *current_stream();
    let moving_arrows = &mut *current_moving_arrows();
    
    moving_arrows.clear();
    let [x, y, _, _] = stream.rect;
    let [n, m] = stream.nm(SAMPLE_SIZE);
    for i in range(0, n) {
        for j in range(0, m) {
            let sx = i as f64 * SAMPLE_SIZE + x;
            let sy = j as f64 * SAMPLE_SIZE + y;
            moving_arrows.push(MovingArrow {
                arrow: Arrow {
                    pos: [sx, sy],
                    dir: stream.at([sx, sy]),
                },
                start_pos: [sx, sy],
                time: 0.0,
            });
        }
    }
}

pub fn update_stream(dt: f64) {
    use current_stream;
    use settings::stream::PHASE_VEL;
    use piston::vecmath::consts::Radians;
   
    let shift = dt * PHASE_VEL * Radians::_360();
    let stream = &mut *current_stream();
    for arrow_phase in stream.arrow_phases.iter_mut() {
        *arrow_phase += shift;
    }
}

pub fn update_moving_arrows(dt: f64) {
    use current_stream;
    use current_moving_arrows;
    use piston::vecmath::vec2_add as add;
    use piston::vecmath::vec2_scale as scale;
    use piston::vecmath::vec2_len as len;
    use settings::stream::{ MOVING_ARROW_TIME_SPAN, SPEEDUP };

    let stream = &mut *current_stream();
    let moving_arrows = &mut *current_moving_arrows();
    for moving_arrow in moving_arrows.iter_mut() {
        let arrow = moving_arrow.arrow;
        let diff = scale(arrow.dir, dt * SPEEDUP);
        moving_arrow.arrow.pos = add(arrow.pos, diff);
        moving_arrow.arrow.dir = stream.at(moving_arrow.arrow.pos);
        moving_arrow.time += len(diff);
        if moving_arrow.time > MOVING_ARROW_TIME_SPAN {
            // Reset back to beginning.
            let start_pos = moving_arrow.start_pos;
            let dir = stream.at(start_pos);
            *moving_arrow = MovingArrow {
                arrow: Arrow {
                    pos: start_pos,
                    dir: dir,
                },
                start_pos: start_pos,
                time: 0.0,
            };
        }
    }
}
