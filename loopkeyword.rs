fn main() {
    let mut n: i32 = 0;
    
    loop {
        n += 1;

        if n % 2 == 1 {
            continue;
        }

        if n > 100 {
            break;
        }

        println!("The number n is {}", n);
    }
}