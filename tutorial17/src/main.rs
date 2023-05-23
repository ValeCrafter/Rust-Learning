struct Teacher {
    name: &str,
    age: u8,
    property: &str
}

fn main() {
    let vogel = Teacher {name: "Vogel", age: 100, property: "Ass"};

    println!("Teacher is called {}, he is {} years old and he is {}", vogel.name, vogel.age, vogel.property);
}
