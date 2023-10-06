pub fn run(){
    let age: i32 = 18;
    let check_id = true;
    let know_person_of_age = false;

    if (age >=21 && check_id) || know_person_of_age {
        println!("Bartender: What would you like to drink?");
    }else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave.");
    }else {
        println!("Bartender: I'll need to see your ID.");
    }
    //Short hand
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age {}", is_of_age);
}