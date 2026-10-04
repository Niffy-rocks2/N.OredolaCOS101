use std::io;

fn main() {
    // display the menu
    println!("\n===== THE RESTAURANT MENU =====");
    println!("P   Poundo Yam / Edinkaiko Soup   N3,200");
    println!("F   Fried Rice & Chicken          N3,000");
    println!("A   Amala & Ewedu Soup            N2,500");
    println!("E   Eba & Egusi Soup              N2,000");
    println!("W   White Rice & Stew             N2,500");

    // read the food type
    println!("\nEnter food type (P, F, A, E or W): ");
    let mut food = String::new();
    io::stdin()
        .read_line(&mut food)
        .expect("Failed to read food type");
    let food = food.trim().to_uppercase();

    // read the quantity
    println!("Enter quantity: ");
    let mut qty_input = String::new();
    io::stdin()
        .read_line(&mut qty_input)
        .expect("Failed to read quantity");
    let quantity: u32 = qty_input.trim().parse().expect("Please enter a whole number");

    // decide the price from the letter
    let name: &str;
    let price: u32;

    if food == "P" {
        name = "Poundo Yam / Edinkaiko Soup";
        price = 3200;
    } else if food == "F" {
        name = "Fried Rice & Chicken";
        price = 3000;
    } else if food == "A" {
        name = "Amala & Ewedu Soup";
        price = 2500;
    } else if food == "E" {
        name = "Eba & Egusi Soup";
        price = 2000;
    } else if food == "W" {
        name = "White Rice & Stew";
        price = 2500;
    } else {
        println!("Invalid food type!");
        return;
    }

    // compute the total
    let mut total = price * quantity;
    println!("\nOrder: {} x {}", quantity, name);
    println!("Charge: N{}", total);

    // discount if the total is greater than N10,000
    if total > 10000 {
        let discount = total * 5 / 100;
        total = total - discount;
        println!("Discount (5%): N{}", discount);
    }

    println!("Total to pay: N{}", total);
}