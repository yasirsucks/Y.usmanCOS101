use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    println!("Enter value for a: ");
    io::stdin().read_line(&mut input_a).expect("Failed to read input");
    let a: f32 = input_a.trim().parse().expect("Not a valid number");

    println!("Enter value for b: ");
    io::stdin().read_line(&mut input_b).expect("Failed to read input");
    let b: f32 = input_b.trim().parse().expect("Not a valid number");

    println!("Enter value for c: ");
    io::stdin().read_line(&mut input_c).expect("Failed to read input");
    let c: f32 = input_c.trim().parse().expect("Not a valid number");

    let root_part = (b*b - 4.0*a*c).sqrt(); 
    let root1 = (-b + root_part) / (2.0 * a);
    let root2 = (-b - root_part) / (2.0 * a);

    println!("The first root is: {}", root1);
    println!("The second root is: {}", root2);
}
