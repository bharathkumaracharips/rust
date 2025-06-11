pub fn sum_of_natural_numbers(){
    let mut n = String::new();
    println!("Enter a number: ");
    let _=std::io::stdin().read_line(&mut n);
    let  n :i32 = n.trim().parse().unwrap_or(0);
    let mut sum = 0;
    for i in 1..=n{
        sum += i;
    }
    println!("The sum of first {} natural numbers is: {}", n, sum);
}