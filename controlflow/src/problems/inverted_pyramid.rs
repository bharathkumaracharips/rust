pub fn inverted_pyramid() {
    println!("Enter a value for n: ");
    let mut n = String::new();
    let _ = std::io::stdin().read_line(&mut n);
    let n: i32 = n.trim().parse().unwrap_or(0);

    for i in 0..n {
        // Print spaces
        for _ in 0..i {
            print!(" ");
        }
        // Print stars
        for _ in 0..(2 * (n - i) - 1) {
            print!("*");
        }
        println!();
    }
}

fn main() {
    inverted_pyramid();
}