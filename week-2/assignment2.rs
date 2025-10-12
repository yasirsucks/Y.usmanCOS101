fn main() {
    // Amounts for each item
    let toshiba: f64 = 450000.0 * 2.0;
    let mac: f64 = 1500000.0 ;
    let hp: f64 = 750000.0 * 3.0;
    let dell: f64 = 2850000.0 * 3.0;
    let acer: f64 = 250000.0;

    // Calculate sum
    let sum = toshiba + mac + hp + dell + acer;

    // Calculate average
    let average = sum / 10.0;

    // Display results
    println!("Total Sales Amount = ₦{}", sum);
    println!("Average Sales Amount = ₦{}", average);
}
