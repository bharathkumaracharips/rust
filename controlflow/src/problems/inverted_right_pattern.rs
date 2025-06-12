pub fn inverted_right(){
    println!("enter the value of n : ");
    let mut n = String::new();
    let _=std::io::stdin().read_line(&mut n);
    let n:i32 = n.trim().parse().unwrap_or(0);

    for i in 0..n{
        for _ in i..n{
            print!("* ");
        }
        println!();
    }
}
fn main(){
    inverted_right();
}