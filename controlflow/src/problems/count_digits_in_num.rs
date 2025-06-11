pub fn digits_count(){
    let mut n=String::new();
    println!("Enter a number: ");
    let _=std::io::stdin().read_line(&mut n);
    let n:i32=n.trim().parse().unwrap_or(0);
    let mut count =0;
    let mut num = n.abs(); // Handle negative numbers by taking absolute value
    if num ==0 {
        count =1;
    }else{
        while num >0{
            num = num / 10;
            count += 1;
        }
    }
    println!("The number of digits in {} is: {}", n, count);
}