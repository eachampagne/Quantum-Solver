/// Elizabeth Champagne
/// graphers.rs
/// This file contains the functions that interface with a plotting library, as well as utility functions to rearrange data
/// into the proper types to be graphed.
/// This works, but I have not added legends and other labels. 

use plotters::prelude::*;

///Most of this code is borrowed/adapted from the plotters library tutorial
///https://plotters-rs.github.io/book/basic/basic_data_plotting.html
///I've never used this library before so I'm still not completely certain how it works
pub fn test_graph(x:&Vec<f64>, psi:&Vec<f64>, filename:&str) {
    println!("Graphing function");
    let path = "images/".to_owned()+filename+".png";
    let root_drawing_area = BitMapBackend::new(&path, (1024,768))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let (min, max) = get_bounds(&psi);
    let (xmin, xmax) = get_bounds(&x);

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Wavefunction vs x", ("sans-serif", 40))
        .build_cartesian_2d(xmin-1.0..xmax+1.0, min-1.0..max+1.0)
        .unwrap();

    //Apparently this takes forever to run with large y axis
    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(zip_coordinates(&x, &psi).into_iter(),&BLUE))
        .unwrap();
        
    root_drawing_area.present().expect("Unable to write result to file, please make sure target dir exists under current dir");
}

///A function to make a vector of tuples out of two separate vectors for the grapher
///Technically I should check to make certain x and psi have the same number of elements
pub fn zip_coordinates(x:&Vec<f64>, psi:&Vec<f64>) -> Vec<(f64,f64)> {
    let mut coors = Vec::<(f64,f64)>::new();

    for i in 0..x.len() {
        coors.push((x[i],psi[i]));
    }
    //println!("{:?}", coors);

    coors
}

///Find highest and lowest values of psi so I can calibrate my graphs correctly
pub fn get_bounds(psi:&Vec<f64>) -> (f64,f64) {
    let mut min:f64 = 0.0;
    let mut max:f64 = 0.0;

    for i in psi {
        if *i > max {max = *i;}
        if *i < min {min = *i;}
    }

    (min, max)
}

///A test function to make certain the library is set up properly and producing graphs
///Left in for testing purposes, in case I break something later
pub fn test() {
    println!("Test graphing function");
    let root_drawing_area = BitMapBackend::new("images/01.png", (1024,768))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
        .unwrap();

    chart.draw_series(LineSeries::new((-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),&RED))
        .unwrap();

    root_drawing_area.present().expect("Unable to write result to file, please make sure target dir exists under current dir");
}