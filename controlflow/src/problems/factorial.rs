pub fn fact(){
    let mut n = String::new();
    println!("Enter a number to calculate its factorial: ");
    let _=std::io::stdin().read_line(&mut n);
    let mut n: i32 = n.trim().parse().unwrap_or(0);
    let mut factorial: i32 = 1;
    while n>0{
        factorial *= n;
        n -= 1;
    }
    println!("The factorial is: {}", factorial);
}