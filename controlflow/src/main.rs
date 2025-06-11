fn main(){
    let mut option = String::new();
    println!("Choose an option :");
    println!("1. If Example");
    println!("2. For Loop Example");
    println!("3. Types incompactable in if example");
    println!("4. Loop Example");
    println!("5. While Loop Example");
    println!("6. Sum of First n Natural Numbers");
    println!("7. Print Even Numbers from 1 to n");
    println!("8. Factorial of a Number");
    println!("9. Reverse of a Number");
    println!("10. Count Digits in a Number");
    println!("11. Fibonacci series");
    let _ = std::io::stdin().read_line(&mut option);
    let option: u64 = option.trim().parse().unwrap_or(0);
    if option == 1 {
        let if_result = controlflow::if_else::if_example::if_example();
        println!("The result of the if example is: {}", if_result);
    } else if option == 2 {
        let for_result = controlflow::for_loop::forloop::loop_test();
        println!("The result of the for loop example is: {}", for_result);
    } else if option == 3 {
        let _if_example_2_result = controlflow::if_else::if_example_2::if_example_2();
    } else if option == 4 {
        controlflow::loop_example::loop_example1::loop_example_1();
    } else if option == 5 {
        controlflow::while_loop::while_example::while_example();
    } else if option == 6{
        controlflow::problems::sum_of_naturals_numbers::sum_of_natural_numbers();
    } else if option == 7 {
        controlflow::problems::print_even_numbers_from_1_to_n::even_to_n();
    } else if option ==8 {
        controlflow::problems::factorial::fact();
    } else if option ==9{
        controlflow::problems::reverse_of_int::reverse();
    } else if option == 10 {
        controlflow::problems::count_digits_in_num::digits_count();
    } else if option == 11 {
        controlflow::problems::fibonacci::fib();
    }
    else {
        println!("Invalid option selected.");
        return;
    }
}