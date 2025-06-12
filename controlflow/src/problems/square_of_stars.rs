pub fn square(){
    println!("enter the value of N : ");
    let mut n = String::new();
    let _=std::io::stdin().read_line(&mut n);
    let n:i32 = n.trim().parse().unwrap_or(0);

    for _ in 0..n{
        for _ in 0..n{
            print!("* ");
        }
        println!();
    }
}