pub fn right(){
    println!("enter a value of N : ");
    let mut n = String::new();
    let _=std::io::stdin().read_line(&mut n);
    let n:i32 = n.trim().parse().unwrap_or(0);

    for i in 1..=n{
        for _ in 0..i{
            print!("* ");
        }
        println!();
    }
}
fn main(){
    right();
}