//fixed type arrays
pub fn print_arrays(arr: [i32; 6]) {
    let mut i = 1;
    for n in arr {
        println!("The {} number is {}", i, n);
        i += 1;
    }
}

pub fn sort_and_print_arrays(mut arr: [i32; 6]) {
    arr.sort();
    let mut i = 1;
    for n in arr {
        println!("The {} number is {}", i, n);
        i += 1;
    }
}

pub fn sort_and_print_in_descending_order(mut arr: [i32; 6]) {
    arr.sort_by(|a, b| b.cmp(a));
    let mut i = 1;
    for n in arr {
        println!("The {} number is {}", i, n);
        i += 1;
    }
}

pub fn slice_elements(arr: [i32; 6], number: usize) {
    let sliced_ele = &arr[0..number];
    println!("slice elements: {:?}", sliced_ele);
}
