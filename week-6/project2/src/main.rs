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
