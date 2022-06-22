struct Person {
    name: String,
    age: u32,
    address: String,
}

impl Person {
    fn new_person(name: String, age: u32, address: String) -> Person {
        Person { name, age, address }
    }
}
fn main() {
    // let user = (24,String::from("Nairobi"),String::from("onchez"));
    // let (name,age,address) =user;
    // let mut p1 = Person {
    //     name: String::from("onchez"),
    //     age: 24,
    //     address: String::from("Nairobi"),
    // };

    let mut p1 = Person::new_person("onchez".to_string(), 34, "Nairobi".to_string());
    println!("the name {}", &p1.name);
    // let _p2 = Person {
    //     name: String::from("Dina"),
    //     age: 13,
    //     address: String::from("Kisumu"),
    // };

    p1.name = String::from("kibe");
    println!("the name {}", &p1.name);
}
