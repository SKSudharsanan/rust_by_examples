mod arrays;
mod literals;
mod scalar;
mod tuples;
fn main() {
    //variable declaration
    //variables are immutable by default
    let number1 = 200;
    let number2 = 40;
    //two types of type annotation - regular and suffix
    let number3: f64 = 21.2;
    let number4 = 321.4f64;
    //the following will be errored since variables are immutable by default
    //let number5 = 120u32;
    //these can overriden by using the mut keyword
    //the above line should be commented out and In below line, remove comments
    let mut number5 = 120u32;
    number5 = 320u32;
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    // Error! The type of a variable can't be changed.
    // mutable = true;
    // Variables can be overwritten with shadowing.
    let mutable = true;
    //two types - scalar and compound
    scalar::showcase();
    literals::using_underscore_for_readablity();
    literals::print_scientific_notation();
    //scalar operators
    literals::arthimetic_operator_showcase(number1, number2);
    literals::boolean_gates_showcase();
    literals::bitwise_operations();
    //compound types
    //print reverse
    tuples::print_reverse_pair();

    //print a matrix
    tuples::print_matrix_struct();

    //destructuring tuples
    tuples::destructuring_tuples();

    //creating one item tuples
    tuples::create_one_item_tuples();

    //tuple of tuples
    tuples::print_tuple_of_tuples();
    // create a mutable array
    println!("create a mutable array and print");
    let mut arr: [i32; 6] = [123, 567, 789, 890, 43, 265];
    arrays::print_arrays(arr);
    println!("sorted arrays");
    arrays::sort_and_print_arrays(arr);
    println!("print in descending order");
    arrays::sort_and_print_in_descending_order(arr);
    println!("get top three elements");
    arrays::slice_elements(arr, 3);
}
