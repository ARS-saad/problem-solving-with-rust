pub fn run(){
    let name: String = "Saad";
    let mut age = 20;
    
    println!("My name is {} and my age is {}", name, age);
    age = 50;
    
    println!("My name is {} and my age is {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let ( my_name, my_age ) = ("Saad", 20);
    println!("{} is {}", my_name, my_age);
}