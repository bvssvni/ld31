
pub struct BloodBar(pub f64);

pub fn decrease(d: f64) {
    use current_blood_bar;

    let &mut BloodBar(ref mut val) = unsafe { &mut *current_blood_bar() };
    *val = (*val - d).max(0.0);
}

pub fn update_blood_bar(dt: f64) {
    use settings::blood_bar::DEC_VAL;
    
    decrease(dt * DEC_VAL);
}

