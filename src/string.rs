pub fn run(){
    let mut hi_now = String::from("Hello%$@364566 ");

    println!("Length: {}", hi_now.len());

    hi_now.push('W');
    hi_now.push_str("orld!");

    println!("Capacity: {}", hi_now.capacity());
    println!("Is Empty: {}", hi_now.is_empty());
    println!("Contains 'World' {}", hi_now.contains("World"));
    println!("Replace: {}", hi_now.replace("Hello", "there")); 

    for word in hi_now.chars(){             //split_ascii_whitespace
        if word >= 'a' && word <= 'z'{println!("OK {}", word);}
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion testing
    assert_eq!(2, s.len());

    println!("{}", s);

    println!("{}",hi_now);
}