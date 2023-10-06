pub fn run(){
    println!("Print from this in print.rs file");
    println!("Number: {}",1);
    println!("{0} is from {1} and {0} like to {2}","Brad","Mass","code");

    println!(
        "{name} like to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    println!("Binary: {:b} Hex: {:x} octal: {:o}", 10, 10, 10);

    println!("{:?}", (12, true, "Hello"));
}