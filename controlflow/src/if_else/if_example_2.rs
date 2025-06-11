//value types incompactable
pub fn if_example_2() {
    let condition = false; // Change this to false to see the other branch
    if condition {
        println!("The number is 5");


    // Uncommenting the following lines will cause a compilation error due to type mismatch
    // important *** 🚨 *** 
    // else {
    //     "SIX" // This value will be used if condition is false
    // };

    
    } else {
        println!("invalid type in if example 2");
    }
}