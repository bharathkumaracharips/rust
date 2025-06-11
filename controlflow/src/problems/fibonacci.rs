pub fn fib(){
    let mut n = String::new();
    println!("enter a number : ");
    let _=std::io::stdin().read_line(&mut n);
    let n:i32 = n.trim().parse().unwrap_or(0);
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        print!("{} ", a);
        let temp = a + b;
        a = b;
        b = temp;
    }
    println!(" ");
}