use std::io;

fn main() {
    let mut experience = String::new();
    let mut input_age = String::new();

      println!("Are you experienced? (yes or no): ");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase(); // make it lowercase to handle YES/Yes/yes

    println!("Enter your age: ");
    io::stdin().read_line(&mut input_age).expect("Failed to read input");
    let age: i32 = input_age.trim().parse().expect("Not a valid number");

    if experience == "yes" && age >= 40 {
        println!("Your annual incentive is ₦1,560,000");

    } else if experience == "yes" && age >= 30 && age < 40 {
        println!("Your annual incentive is ₦1,480,000");

    } else if experience == "yes" && age < 28 {
        println!("Your annual incentive is ₦1,300,000");

    } else if experience == "no" {
        println!("Your annual incentive is ₦100,000");

    } else {
        println!("Input does not match any category.");
    }
}
