use std::io;

fn main() {
    println!("Rust program that displays the following menu for food items and taking order");

    println!("\nWelcome to food canteen");
    println!("\n| code  || menu                       || price |");
    println!("| a     || pounded yam/edinkaiko soup || 3200  |");
    println!("| b     || fried rice and chicken     || 3000  |");
    println!("| c     || pounded yam/edinkaiko soup || 2500  |");
    println!("| d     || eba and egusi soup         || 2000  |");
    println!("| e     || white rice and stew        || 2500  |");

    println!("\nChoose your choice of food code as displayed above:");
    let mut firstpick = String::new();
    io::stdin()
        .read_line(&mut firstpick)
        .expect("not a valid string");
    let code = firstpick.trim().to_lowercase();

    println!("\nHow many?");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("not a valid string");
    let quantity: f32 = input2
        .trim()
        .parse()
        .expect("not a valid number");

    let price = match code.as_str() {
        "a" => 3200.0,
        "b" => 3000.0,
        "c" => 2500.0,
        "d" => 2000.0,
        "e" => 2500.0,
        _ => {
            println!("please input a valid item code");
            return;
        }
    };

    let total = price * quantity;
    println!("\nThe total price is: ₦{}", total);

let total = price * quantity;
let discount = total * (0.5 * total);
if total > 10000.0 {
    println!(
        "Your total is passed 10000. You have been given a discount of 0.5%. Here is your balance: {}",
        discount
    );
} else if total <= 10000.0 {
    println!("Your total is {}", total);
}

println!("Do you wish to continue (y/n)?");
let mut input3 = String::new();
io::stdin()
    .read_line(&mut input3)
    .expect("not a valid string");

// let answer = match input3.trim() {
//     "y" => "yes",
//     "n" => "no",
//     _ => {
//         println!("Please input either y/n");
//         break;
//     }
// }

// if {
}
