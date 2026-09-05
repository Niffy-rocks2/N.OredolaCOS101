fn main() {
    println!("Welcome to total sum and average calculator");
    println!();

    let toshiba: f64 = 450_000.00;
    let mac: f64 = 1_500_000.00;
    let hp: f64 = 750_000.00;
    let dell: f64 = 2_850_000.00;
    let acer: f64 = 250_000.00;

    let sum = toshiba + mac + hp + dell + acer;
    let count: f64 = 5.0;
    let average = sum / count;

    println!("Total sum of sales is {}", sum);
    println!("Average sales is {}", average);

    println!();
    println!("Thank you for using the calculator");
}