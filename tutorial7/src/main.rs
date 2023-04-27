fn main() {
    let mut n = 0;

    loop {
        n += 1;
        if n > 10 {
            break;
        }
        if n == 5 {
            continue;
        }

        println!("{}", n);
    }
}
