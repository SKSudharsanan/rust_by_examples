pub fn using_underscore_for_readablity() {
    println!("One million is written as {}", 1_000_000u32);
}

pub fn arthimetic_operator_showcase(a: u32, b: u32) {
    add(a, b);
    sub(a, b);
    mul(a, b);
    div(a, b);
}

//arithmetic operations - add
pub fn add(a: u32, b: u32) {
    println!("{} + {} = {}", a, b, a + b);
}

pub fn sub(a: u32, b: u32) {
    println!("{} - {} = {}", a, b, a - b);
}

pub fn mul(a: u32, b: u32) {
    println!("{} * {} = {}", a, b, a * b);
}

pub fn div(a: u32, b: u32) {
    println!("{} / {} = {}", a, b, a * b);
}

pub fn boolean_gates_showcase() {
    boolean_and();
    boolean_or();
    boolean_not();
}

//using scientific notations
pub fn print_scientific_notation() {
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
}

//boolean operations
pub fn boolean_and() {
    println!("true AND false is {}", true && false);
}

pub fn boolean_or() {
    println!("true or false is {}", true || false);
}

pub fn boolean_not() {
    println!("NOT true is {}", !true);
}

pub fn bitwise_operations() {
    //bitwise and
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    //bitwise or
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    //bitwise xor
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    //bitwise left shift
    println!("1 << 5 is {}", 1u32 << 5);
    //bitwise right shift
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
