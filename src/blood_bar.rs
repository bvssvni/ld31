
pub struct BloodBar(pub f64);

pub fn update_blood_bar(dt: f64) {
    use current_blood_bar;
    use settings::blood_bar::DEC_VAL;
    use std::num::FloatMath;
   
    let &BloodBar(val) = &mut *current_blood_bar();
    let new_val = (val - DEC_VAL * dt).max(0.0);
    *current_blood_bar() = BloodBar(new_val);
}
