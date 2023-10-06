pub fn run(){
    // let arr1 = [1, 2, 3];
    // let arr2 = arr1;

    let vac1 = vec![1, 2, 3];
    let vac2 = &vac1;

    println!("Values: {:?}", (&vac1, vac2));
}