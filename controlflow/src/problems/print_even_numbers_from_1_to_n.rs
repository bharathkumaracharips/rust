pub fn even_to_n(){
    let mut n= String::new();
    println!("Enter a number: ");
    let _=std::io::stdin().read_line(&mut n);
    let n:i32 = n.trim().parse().unwrap_or(0);
    for i in 1..=n{
        if i>0 && i%2==0{
            println!("Even number : {}",i);
        } 
    }
}