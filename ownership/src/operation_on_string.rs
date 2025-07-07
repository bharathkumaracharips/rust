fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership of `s1` moved to `s2`

    // println!("{}", s1); ❌ Error! s1 is invalid
    println!("{}", s1); // ✅ Works fine
}