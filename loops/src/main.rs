use std::io;

fn fibonacci(nth: u32) -> u32 {
    let (mut x, mut y) = (1, 1);
    let mut tmp;

    for _ in 3..(nth+1) {
        tmp = x;
        x = x + y;
        y = tmp;
    }

    x
}

fn main() {
    let val: u32;
    
    loop {
        println!("Please input nth of fibonacci.");
        let mut nth = String::new();
        io::stdin().read_line(&mut nth)
            .expect("Failed to read nth");
        val = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    println!("{}", fibonacci(val));
}
