pub fn run(){
    // Default is i32
    let x =1;

    let y = 2.5;

    let z: i64 = 52466545;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    
    let is = true;
    //Boolean expression
    let is_greater: bool = 10<5;
    let a1 = "a";
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is, is_greater, a1, face));
}

// use std::io;
// fn read<T: std::str::FromStr>() -> Vec<T> {
//     let mut buf = String::new();
//     io::stdin().read_line(&mut buf).unwrap();
//     buf.trim().split(' ').flat_map(str::parse).collect()
// }
// fn solve(i:usize, m:usize, a:&Vec<usize>) -> bool{
//     if m == 0{
//         return true;
//     }
//     if i >= m{
//         return false;
//     }
//     return solve(i + 1, m, a) || solve(i + 1, m - a[i], a);
// }
// fn main() {
//     let _n = read::<usize>();
//     let a = read::<usize>();
//     let q = read::<usize>();
//     let m = read::<usize>();
//     for i in 0..q[0]{
//         if solve(0, m[i],&a) == true{
//             println!("yes");
//         }else{
//             println!("no");
//         }
//     } 
// }