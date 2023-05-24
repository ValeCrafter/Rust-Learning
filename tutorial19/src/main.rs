struct Teacher {
    name: String,
    age: u8,
    property: String
}

fn main() {
    let vogel = Teacher {name: "Vogel".to_string(), age: 100, property: "Ass".to_string()};
    let mut jenny = Teacher{name: "Jenny".to_string(), age: 68, property: "a Wixxer".to_string()};
    print_teacher(&vogel);
    print_teacher_mut(&mut jenny);
}

fn print_teacher(t:&Teacher){
    println!("Teacher is called {}, he is {} years old and he is {}", t.name, t.age, t.property);
}

fn print_teacher_mut(t:&mut Teacher){
    t.age = 69;
    println!("Teacher is called {}, he is {} years old and he is {}", t.name, t.age, t.property);
}