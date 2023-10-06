use std::io;

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let mut i: i16 = 0;

    let mut chars: Vec<char> = user_input.chars().collect();
    chars.sort();
    chars.dedup();       

    println!("{:?}", chars);
    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        // string.push_str(", ");
    }

    println!("{}",user_input);
    // // The trimmed string is a slice to the original string, hence no new
    // // allocation is performed
    // let chars_to_trim: &[char] = &[' ',','];
    // let trimmed_str: &str = string.trim_matches(chars_to_trim);

    for word in string.chars(){             
        if word >= 'a' && word <= 'z'{ i+=1;}
    }

    println!("Output: {}", i);
    // // println!("Used characters: {}", trimmed_str);
    // // println!("{:?}", trimmed_str.len());

    // // while let valu = stdin.read_line(&mut user_input){
    // //     if user_input.chars() {println!("Its ok {}", user_input);}
    // //     // if(valu.len() == input.trim().len()){ break;}
    // // }
    // // println!("You typed: {}", user_input);
    Ok(())
}