pub fn run(){
    //let mut count = 1;

    //Infinite loop
    // loop{
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    //while loop
    // while count <= 100 {
    //     if count % 15 == 0{
    //         println!("fizzbuzz");
    //     }else if count % 5 == 0 {
    //         println!("fizz");
    //     }else if count % 3 == 0 {
    //         println!("buzz");
    //     }else {
    //         println!("{}", count);
    //     }
    //     //Inc
    //     count += 1;
    // }

    //for range
    for x in 1..100{
        if x % 15 == 0{
            println!("fizzbuzz");
        }else if x % 5 == 0 {
            println!("fizz");
        }else if x % 3 == 0 {
            println!("buzz");
        }else {
            println!("{}", x);
        }
    }
}