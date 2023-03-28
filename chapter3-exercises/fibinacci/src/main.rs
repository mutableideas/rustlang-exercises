use std::io;

fn main() {
    let value = loop {
        println!("finibnnaci ?");
        
        let mut fib_number = String::new();
        io::stdin().read_line(&mut fib_number)
            .expect("Failed to readline.");

        let f: i32 = match fib_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid value");
                continue;
            }
        };

        break fib(f)
    };

    println!("Fib Number is {}", value);
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}
