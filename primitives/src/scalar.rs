pub fn showcase() {
    //signed integers start with i - i8, i16, i32, i64, i128, isize(based on the arch of your system)
    let number6: i32 = 54;
    let number7: i64 = -43;
    //by default its i32
    let number8 = -76; //type here is i32
                       //unsigned integers start with u - u8, u16, u32, u64, u128, isize(based on the arch of your system)
    let number9: u32 = 54;
    let number10: u64 = 43;
    //floating numbers - f32,f64
    let float1: f32 = 32.0;
    let float2 = 65.4; // default value is f64
                       //char
    let name: char = 'a';
    //bool
    let is_eligible: bool = true;
    //unit tuple
    let unit_tuple = ();
}
