pub fn loop_test() -> u64 {
    let mut n = String::new();
    println!("Enter a number n:");
    let _=std::io::stdin().read_line(&mut n);
    let n: u64 = n.trim().parse().unwrap_or(0);
    println!("Choose an option (1 or 2):");
    println!("1. Sum numbers from 1 to n-1");
    println!("2. Sum numbers from 1 to n (inclusive)");
    let mut option = String::new();
    let _=std::io::stdin().read_line(&mut option);
    let option: u64 = option.trim().parse().unwrap_or(0);
    if option == 1 { 
        // Using a for loop to sum numbers from 1 to n-1
        let mut sum_1 = 0;
        for i in 1..n {
            sum_1 += i;
        }
        sum_1
    } else {
        // Using a for loop to sum numbers from 1 to n (inclusive)
        let mut sum_2 = 0;
        for i in 1..=n {
            sum_2 += i;
        }
        sum_2
    }
}