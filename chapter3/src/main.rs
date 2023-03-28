fn main() {
    // Working mutablility
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The number of spaces is {}", spaces);

    let guess: u32 = "42".parse().expect("Not A number!");
    println!("The number converted is {}", guess);

    let signed_int: i32 = -5;
    let unsigned_int: u32 = 5;
    println!("signed int {}, unsigned int {}", signed_int, unsigned_int);

    let tup: (i32, i32) = (175, 36);

    // destructuring similar to JS / Typescript
    let (weight, waist) = tup;

    println!("weight: {}, waist: {}", weight, waist);

    do_stuff(19);
    expressions_statements();

    let r_value = returned_values();
    println!("Early Return {}", r_value);

    let is_older_than_45 = control_flow(32);
    println!("Is older than 45 {}", is_older_than_45);

    let if_expression = if_expression_to_statement();
    println!("If Expression Statement {}", if_expression);

    let ten_loop = loop_to_ten();
    println!("Loop to ten {}", ten_loop);

    ten_while();
    loop_for_collection();
    loop_for_rev_range();
}

fn do_stuff(write_this: i8) {
    println!("This is my super duper sweet do_stuff function {}", write_this);
}

fn expressions_statements() {
    // statement 
    let x = 5;

    let y = {
        let i = 10;
        x + i
    };

    println!("Expression statement {}", y);
}

fn returned_values() -> i32 {
    12
}

fn control_flow(i: i32) -> bool {
    if i > 45 {
        return true;
    }

    false
}

fn if_expression_to_statement() -> i32 {
    let x = if true {
        6
    } else {
        5
    };

    x
}

fn loop_to_ten() -> u32 {
    let mut counter = 0;

    return loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    }
}

fn ten_while() {
    let mut index = 0;

    while index < 10 {
        index += 1;
        println!("while looping: {}", index);
    }
}

fn loop_for_collection() {
    let collection = [1, 3, 56, 9, 25, 10];

    for number in collection.iter() {
        println!("Number for iter() {}", number);
    }
}

fn loop_for_rev_range() {
    for number in (1..5).rev() {
        println!("Number for rev() {}", number);
    }
}
