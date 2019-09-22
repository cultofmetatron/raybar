pub enum FlCompare {
    GT,
    LT,
    EQ,
}

pub fn fl_cmp(a: f64, b: f64) -> FlCompare {
    let epsilon: f64 = 0.0001;
    let diff = a - b;
    if diff.abs() < epsilon {
        return FlCompare::EQ;
    } else if diff > 0.0 {
        return FlCompare::GT;
    } else {
        return FlCompare::LT;
    }
}

pub fn fl_eq(a: f64, b: f64) -> bool {
    match fl_cmp(a, b) {
        FlCompare::EQ => true,
        _ => false,
    }
}
