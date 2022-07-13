# Chatper 2 Summary

## Key Takeaways
1. Variables in `Rust` as `immutable` by default.
2. `&` indicates the an argument is a reference.
3. Handling potential failures with a return type `.expect("Text goest here")`.
4. `println!` macro uses placeholders via the `{}` syntax within a string. `println!("Hi {}!", "you")`, will print `"Hi you!"`.
5. Adding dependencies e.g. `rand = "0.3.14"` is part of the `cargo.toml` file under `[dependencies]`.
``` toml
[dependencies]
rand = "0.3.14"
```
6.`Cargo.lock` similar to `package.lock` in `npm` for reproducible builds.
7. Updating crates `cargo update`.  Be explicit when calling the next minor version e.g. `0.3.14` -> `0.4.0`.
8. `loop` runs with `continue` and stops with `break`.
9. `match` provides flow control based on comparing a `Result` against a matching `enum`.

``` rust
 let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue
  };

  match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too Small"),
    Ordering::Greater => println!("That's too high!"),
    Ordering::Equal => {
        println!("That's Right!");
        break;
    }
  }
```
