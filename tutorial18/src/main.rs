struct Teacher ( String, u8, String );

fn main() {
    let mut vogel = Teacher ("Vogel".to_string(), 100, "Ass".to_string());

    vogel.2 = "bad".to_string();

    println!("Teacher is called {}, he is {} years old and he is {}", vogel.0, vogel.1, vogel.2);
}
