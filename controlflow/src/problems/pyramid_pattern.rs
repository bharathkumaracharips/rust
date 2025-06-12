pub fn pyramid() {
    println!("Enter a value:");
    let mut n = String::new();
    let _ = std::io::stdin().read_line(&mut n);
    let n: i32 = n.trim().parse().unwrap_or(0);

    for i in 1..=n {
        // Print spaces
        for _ in 0..(n - i) {
            print!(" ");
        }
        // Print stars
        for _ in 0..(2 * i - 1) {
            print!("*");
        }
        // Move to next line
        println!();
    }
}

fn main(){
    pyramid();
}