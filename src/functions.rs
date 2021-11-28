use crate::potentials::*;

pub fn fn_test() {
    println!("Hello, world!");
    println!("{}",lin_v(-2.0));
    println!("{}",lin_v(-1.0));
    println!("{}",lin_v(0.0));
    println!("{}",lin_v(1.0));
    println!("{}",lin_v(2.0));
}

///x1 will pretty much always be 0.0
///Returns value of divergence from 0 at x2
pub fn solve_schrod(testEnergy:f64, psi1:f64, d_dt_1:f64, d2_dt2_1:f64, x1:f64, x2:f64, no_steps:u32, V:&dyn Fn(f64) -> f64) -> f64 {
    let dx:f64 = (x2-x1) / (no_steps as f64); //Check for off by one errors
    let mut d2_dt2:Vec<f64> = Vec::<f64>::new();
    let mut d_dt:Vec<f64> = Vec::<f64>::new();
    let mut psi:Vec<f64> = Vec::<f64>::new();
    let mut x:Vec<f64> = Vec::<f64>::new();

    d2_dt2.push(d2_dt2_1);
    d_dt.push(d_dt_1);
    psi.push(psi1);
    x.push(x1);

    println!("{:?}",d2_dt2);

    println!("{}",V(-2.0));
    println!("{}",V(-1.0));
    println!("{}",V(0.0));
    println!("{}",V(1.0));
    println!("{}",V(2.0));

    return 0.0;
}