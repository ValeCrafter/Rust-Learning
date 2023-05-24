struct Teacher {
    name: String,
    age: u8,
    property: String
}

fn main() {
    let mut vogel = Teacher {name: "Vogel".to_string(), age: 100, property: "Ass".to_string()};

    vogel.age = 120;

    println!("Teacher is called {}, he is {} years old and he is {}", vogel.name, vogel.age, vogel.property);
}
