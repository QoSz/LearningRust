fn main() {
    let mut x: i32 = 1;

    while x < 50 {

        if x % 5 == 0 {
            println!("The number {} is divisible by 5", x);
        }

        x += 1;
    }
}