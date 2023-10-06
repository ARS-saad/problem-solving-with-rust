use std::mem;       //for short cut in 16 line

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];    //mut for change
    
    // let mut number: Vec<&str> = vec!["fghf"];

    // println!("{}", number[0]);
    //Add on
    numbers.push(99);
    numbers.push(55);

    //pop off
    numbers.pop();

    //RE-assign value
    numbers[2]= 20;
    println!("{:?}", numbers);

    //single value
    println!("Single value: {}", numbers[0]);

    println!("Length: {}", numbers.len());

    //Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut(){
        *x*=2;
    }
    println!("Numbers vec: {:?}", numbers);
}