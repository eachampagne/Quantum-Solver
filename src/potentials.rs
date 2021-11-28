pub fn squ_well_v(x:f64) -> f64 {
    if f64::abs(x) < 1.0 {
        0.0
    } else {
        f64::MAX 
    }
}

pub fn lin_v(x:f64) -> f64 {
    let alpha:f64 = 1.0; //in eV per angstrom
    alpha * f64::abs(x)
}

pub fn lin_half_v(x:f64) -> f64 {
    let alpha:f64 = 1.0; //in eV per angstrom
    if x > 0.0 {
        alpha * x
    } else {
        f64::MAX
    }
}

pub fn quad_v(x:f64) -> f64 {
    let alpha:f64 = 3.0; //in eV per angstrom^2
    alpha * x * x
}

pub fn quad_half_v(x:f64) -> f64 {
    let alpha:f64 = 3.0; //in eV per angstrom^2
    if x > 0.0 {
        alpha * x * x
    } else {
        f64::MAX
    }
}

pub fn cub_v(x:f64) -> f64 {
    let alpha:f64 = 5.0; //in eV per angstrom^3
    alpha * f64::abs(x) * x * x
}

pub fn cub_half_v(x:f64) -> f64 {
    let alpha:f64 = 5.0; //in eV per angstrom^3
    if x > 0.0 {
        alpha * x * x * x
    } else {
        f64::MAX
    }
}

pub fn quart_v(x:f64) -> f64 {
    let alpha:f64 = 5.0; //in eV per angstrom^4
    alpha * x * x * x * x
}

pub fn quart_half_v(x:f64) -> f64 {
    let alpha:f64 = 5.0; //in eV per angstrom^4
    if x > 0.0 {
        alpha * x * x * x * x
    } else {
        f64::MAX
    }
}