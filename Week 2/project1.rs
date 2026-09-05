fn main() {
    println!("Welcome to the Compound Interest Calculator!");
    println!();

    let p: f64 = 520_000_000.0; 
    let r: f64 = 10.0;          
    let n: f64 = 5.0;           

    
    let a = p * (1.0 + (r / 100.0)).powf(n);
    println!("Amount after {} years is {}", n, a);

    let ci = a - p;
    println!("Compound Interest is {}", ci);

    println!();
    println!("Thank you for using the Compound Interest Calculator!");
}