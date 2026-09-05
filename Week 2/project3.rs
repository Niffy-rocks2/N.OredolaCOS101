fn main() {
    println!("Welcome to the Depreciation Calculator");
    println!();

    let p: f64 = 210_000.0; // original price of the TV
    let r: f64 = 5.0;       // depreciation rate per annum
    let n: f64 = 3.0;       // number of years

    // depreciation formula: A = P[1 - (R/100)]^n
    let a = p * (1.0 - (r / 100.0)).powf(n);
    println!("Value of the TV after {} years is {}", n, a);

    println!();
    println!("Thank you for using the calculator");
}