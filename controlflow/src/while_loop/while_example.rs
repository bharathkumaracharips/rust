pub fn while_example() {
    let mut n = String::new();
    let mut sum = 0;
    println!("Enter a number n:");
    let _ = std::io::stdin().read_line(&mut n);
    let mut n: u64 = n.trim().parse().unwrap_or(0);
    while n > 0 {
        sum += n;
        n -= 1;
    }
    println!("Final sum: {}", sum);
}