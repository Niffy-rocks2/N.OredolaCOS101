use std::io;
use std::f64::consts::PI;

// Helper: prompt, read a line, and parse it as f64
fn read_f64(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input")
}

fn trapezium_area() -> f64 {
    let height = read_f64("Enter height:");
    let base1 = read_f64("Enter base1:");
    let base2 = read_f64("Enter base2:");
    height / 2.0 * (base1 + base2)
}

fn rhombus_area() -> f64 {
    let diagonal1 = read_f64("Enter diagonal1:");
    let diagonal2 = read_f64("Enter diagonal2:");
    0.5 * diagonal1 * diagonal2
}

fn parallelogram_area() -> f64 {
    let base = read_f64("Enter base:");
    let altitude = read_f64("Enter altitude:");
    base * altitude
}

fn cube_surface_area() -> f64 {
    let side = read_f64("Enter side:");
    6.0 * side * side
}

fn cylinder_volume() -> f64 {
    let radius = read_f64("Enter radius:");
    let height = read_f64("Enter height:");
    PI * radius * radius * height
}

fn main() {
    println!("===== Shape Calculator =====");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");
    println!("Enter your choice (1-5):");

    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read input");
    let choice: u32 = choice_input.trim().parse().expect("Invalid input");

    match choice {
        1 => println!("Area of trapezium = {}", trapezium_area()),
        2 => println!("Area of rhombus = {}", rhombus_area()),
        3 => println!("Area of parallelogram = {}", parallelogram_area()),
        4 => println!("Surface area of cube = {}", cube_surface_area()),
        5 => println!("Volume of cylinder = {}", cylinder_volume()),
        _ => println!("Invalid choice, pick a number from 1 to 5"),
    }
}