// fn main() {
// let mut num = 30;
//     {

//         new(num);
//         num += 2;
//         println!("the value is {}", num);
//     }
//     println!("the number is {}", num);
//     num += 2;
//     println!("the number is {}", num);
// }

// fn new(x: i32) {
//     println!("the value {}", x);
// }

// fn main() {
//     let mut s = String::from("Name");
//     println!("string s : {}", s);
//     s.push_str(":: onchez");
//     println!("new string s : {}", s);
// }

// fn main() {
//     let s1 = String::from("rust");
//     let s2 = s1.clone();
//     println!("the value is  {}", s2);
//     print(s1);
// }

// fn print(mut s: String) {
//     s.push_str(" book");
//     println!("the String {}", s);
// }

fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("the first {} and second {}", hello, world);

    let s_slice = "hello rust";
}
