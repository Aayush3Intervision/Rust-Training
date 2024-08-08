fn main(){
    let height_1:i8 = -130;
    let height_2:i8 = 128;
    println!("The height range of i8 is from -127 to +127 so the values are over-range: {} and {}",height_1,height_2);

    let range_1:u8 = 0;
    let range_2:u8 = 270;
    println!("The height range of u8 is from 0 to 255 so the values are over-range: {} and {}",range_1,range_2);
}

/*
    This shows error because variables are overloaded/overflowed/out of range.
*/