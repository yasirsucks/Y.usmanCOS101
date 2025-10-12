fn main() {
    let p: f64 = 510000.0; // Original price of the TV
    let r: f64 = 5.0;      // Depreciation rate per annum
    let n: f64 = 3.0;      // Time in years

    // Depreciation formula: A = P * (1 - R/100)^n
    let a = p * (1.0 - (r / 100.0)).powf(n);

    println!("Value of the TV after 3 years = ₦{}", a);
}
