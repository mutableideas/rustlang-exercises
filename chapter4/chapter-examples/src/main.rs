fn main() {
    let s1 = String::from("Hootie Hoo!");
    let s2 = take_ownership(s1);

    // This will fail since s1 is borrowed by take ownership
    // println!("value of s1 {}", s1);

    // Prints because now s2 has ownership
    println!("value of s2 {}", s2);

    // borrowing the value
    let size = reference_borrowing(&s2);
    println!("value s2 length: {}", size);

    let first_word_boundary = first_word(&s2);
    println!("first word boundary {}", first_word_boundary);

    let first = first_word_as_slice(&s2);
    println!("First Word: {}", first);
}

fn take_ownership(val: String) -> String {
    val
}


fn reference_borrowing(val: &String) -> usize {
    val.len()
}

fn first_word(word: &String) -> usize {
    let bytes = word.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{ 
            return i;
        }
    }

    word.len()
}

fn first_word_as_slice(word: &String) -> &str {
    let bytes = word.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{ 
            return &word[0..i];
        }
    }

    &word[..]
}