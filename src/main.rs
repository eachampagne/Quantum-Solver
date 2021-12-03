/// Elizabeth Champagne
/// main.rs
/// Everything important

mod functions;
mod potentials;
mod graphers;

use potentials::*;

fn main() {
    let beta:f64 = 2.0 * functions::MASS_E / (functions::HBAR * functions::HBAR);

    let (x_test, psi_test) = functions::solve_schrod(5.0,1.0,1.0,0.0,0.0,10.0,1001,&zero_v);
    graphers::test_graph(&x_test,&psi_test, "test");


    let mut e_values:Vec<f64> = Vec::<f64>::new();
    
    //Boundary conditions
    let psi_1:f64 = 0.0;
    let d_dt_1:f64 = 1.0;

    let tolerance = 0.1;    

    for i in 0..1 {
        let mut within_tolerance:bool = false;
        println!("True:  {}",functions::quad_half_v_energies(0));
        let (mut lower_energy, mut upper_energy) = functions::generate_random_start_energies(i, &functions::quad_half_v_energies);
        //let (mut lower_energy, mut upper_energy) = (0.0, 1.0);
        println!("Lower: {}", lower_energy);
        println!("Upper: {}", upper_energy);
        let mut current_test_energy = (upper_energy + lower_energy) / 2.0;
        println!("{}", current_test_energy);

        let mut x:Vec<f64> = Vec::<f64>::new();
        let mut psi:Vec<f64> = Vec::<f64>::new();

        let mut counter = 0;

        while !within_tolerance {
            let (x_current, psi_current) = functions::solve_schrod(current_test_energy,beta,psi_1,d_dt_1,0.0,5.0,1001,&quad_half_v);

            println!("Psi: {}", *psi_current.last().unwrap());
            
            graphers::test_graph(&x_current,&psi_current,&(counter.to_string()));

            if *psi_current.last().unwrap() < tolerance && *psi_current.last().unwrap() > -tolerance {
                within_tolerance = true;
                x = x_current;
                psi = psi_current;
            } else if *psi_current.last().unwrap() > 0.0 { //I'm not certain which order these go in!
                lower_energy = current_test_energy;
            } else {
                upper_energy = current_test_energy;
            }
            current_test_energy = (upper_energy + lower_energy) / 2.0;
            println!("Current energy: {}", current_test_energy);

            counter += 1;
            println!("Count: {}", counter);
            //drop(x_current);
            //drop(psi_current);
            if counter > 100 {break;}
        }

        e_values.push(current_test_energy);
        graphers::test_graph(&x,&psi,&(i.to_string()));

    }

    

    
}