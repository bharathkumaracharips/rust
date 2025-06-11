pub fn if_example()->u64{
    let condition = true; // Change this to false to see the other branch
    let number = if condition {
        5 // This value will be used if condition is true
    } else {
        10 // This value will be used if condition is false
    };
    number
}