//Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

//Tuple Struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}
impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
             first_name: first.to_string(),
             last_name: last.to_string()
             }
    }

    //Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set first name
    fn set_first_name(&mut self, first: &str){
        self.first_name = first.to_string();
    }

    //Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
} 

pub fn run(){
    // let mut c = Color {
    //     red: 255,
    //     green: 220,
    //     blue: 200
    // };
    // c.red = 205;
    // println!("Color: {} {} {}", c.blue, c.green, c.red);

    // let mut c = Color(230, 150, 35);
    // c.2 = 167;
    // println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Abdur Rahman", "Saad");
    println!("Person {}.", p.full_name());
    p.set_first_name("ARS");
    println!("Person {}...", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}