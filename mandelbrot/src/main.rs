
use rayon::prelude::*;
use num::Complex;


fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize
) -> Vec<Vec<usize>> {
    (0..height).into_par_iter().map(|img_y|{
        (0..width).into_par_iter().map(|img_x|{
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            mandelbrot_at_point(cx, cy, max_iters)
        }).collect()
    }).collect()
}


fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let c = Complex::new(cx, cy);
    let mut z = Complex::new(0.0, 0.0);
    (0..max_iters)
        .take_while(|_| {
            match z.norm() > 2.0 {
                true => false,
                false => {
                    z = z * z + c;
                    true
                }
            }
        })
        .count()
}


fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let line = row.into_par_iter().map(|column| {
            match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            }
        }).collect::<String>();
        println!("{}", line);
    }
}


fn main() {
    let mandelbrot = calculate_mandelbrot(
        1000,  // max iterations
        -2.0,   // lower x bound
        1.0,    // upper x bound
        -1.0,   // lower y bound
        1.0,    // upper y bound
        100,    // width
        24     // height
    );
    render_mandelbrot(mandelbrot);
}

