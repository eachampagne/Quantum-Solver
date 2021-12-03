/// Elizabeth Champagne
/// functions.rs
/// This file contains the main solver, as well as functions to generate test energies.
/// I believe the solver works, but the test energies are bad.

use rand::Rng;

use crate::potentials::*;

pub const HBAR:f64 = 6.582119569e-16; //ev * s
pub const MASS_E:f64 = 5.68563006e-32; //eV *s^2 / angstroms^2

///Test function to make certain files are communicating amongst themselves properly
pub fn fn_test() {
    println!("Hello, world!");
    println!("{}",lin_v(-2.0));
    println!("{}",lin_v(-1.0));
    println!("{}",lin_v(0.0));
    println!("{}",lin_v(1.0));
    println!("{}",lin_v(2.0));
}

///x1 will pretty much always be 0.0
///Returns vector of wave function that can be graphed later
pub fn solve_schrod(test_energy:f64, beta:f64, psi1:f64, d_dt_1:f64, x1:f64, x2:f64, no_steps:usize, potential:&dyn Fn(f64) -> f64) -> (Vec<f64>, Vec<f64>) {
    let dx:f64 = (x2-x1) / (no_steps as f64); //Check for off by one errors
    let mut d2_dt2:Vec<f64> = Vec::<f64>::new();
    let mut d_dt:Vec<f64> = Vec::<f64>::new();
    let mut psi:Vec<f64> = Vec::<f64>::new();
    let mut x:Vec<f64> = Vec::<f64>::new();

    x.push(x1);
    psi.push(psi1);
    d_dt.push(d_dt_1);
    d2_dt2.push(beta * (potential(x[0]) - test_energy) * psi[0]);

    for i in 1..no_steps {
        //CHECK FOR OFF BY ONE ERRORS
        let d_dt_i:f64 = d_dt[i-1] + d2_dt2[i-1] * dx;
        d_dt.push(d_dt_i);
        let psi_i:f64 = psi[i-1] + d_dt[i]*dx + d_dt[i]*dx*dx/2.0;
        psi.push(psi_i);
        let x_i:f64 = x[i-1] + dx;
        x.push(x_i);

        let d2_dt2_i:f64 = beta * (potential(x[i]) - test_energy) * psi[i];
        d2_dt2.push(d2_dt2_i);
    }

    return (x, psi);
}

///Will be used to generate test energies (with noise to avoid starting with the answer)
pub fn quad_half_v_energies(n:i32) -> f64 {
    let alpha:f64 = 3.0;
    let k:f64 = alpha * 2.0;
    let omega:f64 = f64::sqrt(k/MASS_E);
    HBAR * omega * (2.0 * n as f64 + 0.5)
}

//Gives upper and lower starting energies within 10% of true energy
pub fn generate_random_start_energies(n:i32, energies:&dyn Fn(i32) -> f64) -> (f64, f64) {
    let true_energy:f64 = energies(n);
    let mut rng = rand::thread_rng();

    let random_1 = rng.gen_range(0.0..0.1 * true_energy);
    let random_2 = rng.gen_range(0.0..0.1 * true_energy);

    let lower_bound:f64 = true_energy - random_1;
    let upper_bound:f64 = true_energy + random_2;

    (lower_bound, upper_bound)
}