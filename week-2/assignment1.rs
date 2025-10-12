fn main() {
    let p: f64 = 520000000.0; // Principal amount ₦520,000,000
    let r: f64 = 10.0;        // Rate = 10%
    let n: f64 = 5.0;         // Time = 5 years

    // Compound Interest formula
    let a = p * (1.0 + (r / 100.0)).powf(n);
    let ci = a - p;

    println!("Amount after 5 years = ₦{}", a);
    println!("Compound Interest = ₦{}", ci);
}
