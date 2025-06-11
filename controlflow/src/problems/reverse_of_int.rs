pub fn reverse(){
    let mut n=String::new();
    println!("Enter a number: ");
    let _=std::io::stdin().read_line(&mut n);
    let   n: i32 = n.trim().parse().unwrap_or(0);
    let mut reversed:i32 =0;
    let mut num = n.abs(); // Handle negative numbers by taking absolute value
    while num >0{
        let digit = num%10;
        reversed = reversed * 10 + digit;
        num /= 10;
    }
    println!("The reverse of the number {} is: {}", n, reversed);
}