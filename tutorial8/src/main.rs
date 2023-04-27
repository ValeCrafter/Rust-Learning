fn main() {
    let mut n = 0;

    while n <= 80 {
        print! {"n is {}", n};

        if n % 5 == 0 {
            println!(" and a multiple of 5");
        } else {
            println!("");
        }

        n += 1;
    }
}
