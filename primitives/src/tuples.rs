//A tuple is a collection of values of different types.
//Tuples are constructed using parentheses (), and
// each tuple itself is a value with type signature (T1, T2, ...),
//where T1, T2 are the types of its members.
//Functions can use tuples to return multiple values, as tuples can hold any number of values.

//a simple function which accepts two variables and reverse it
pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn longTuple() {
    // A tuple with a bunch of different types.
    let longTuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    // Values can be extracted from the tuple using tuple indexing.
    println!("The first of tuple is {}", longTuple.0);
    println!("The fifth of tuple is {}", longTuple.5);

    // But long Tuples (more than 12 elements) cannot be printed.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error
}

pub fn print_tuple_of_tuples() {
    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);
}

pub fn print_reverse_pair() {
    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));
}

pub fn create_one_item_tuples() {
    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));
}

pub fn destructuring_tuples() {
    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}

pub fn print_matrix_struct() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}
