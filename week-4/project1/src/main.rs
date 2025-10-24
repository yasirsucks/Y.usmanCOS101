use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter value of a: ");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f64 = a.trim().parse().expect("Please enter a number");

    println!("Enter value of b: ");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f64 = b.trim().parse().expect("Please enter a number");

    println!("Enter value of c: ");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f64 = c.trim().parse().expect("Please enter a number");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and different.");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The roots are real and equal.");
        println!("Root = {}", root);
    } else {
        println!("The roots are complex and imaginary.");
    }
}
