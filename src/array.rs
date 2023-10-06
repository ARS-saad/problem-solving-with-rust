use std::mem;       //for short cut in 16 line

pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];    //mut for change

    //RE-assign value
    numbers[2]= 20;
    println!("{:?}", numbers);

    //single value
    println!("Single value: {}", numbers[0]);

    println!("Length: {}", numbers.len());

    //Array are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice 
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}