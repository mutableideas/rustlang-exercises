# Chapter 3 - Common Programming Concepts

## Key Takeaways
1. Variables are immutable by default. Add the `mut` keyword to make it mutable.
``` rust
let mut x = 5;
// Now we can change it to what we want.
x = 6;
```
**Tradeoff** - Consider using mutability in large datasets.  Immutable variables allow for clarity and but offer lower peformance if we need to create a `copy` of the dataset.

2. **Constants** - `const` keyword
  - Naming convent uppercase and underscored e.g. `CONSTANT_NAME`.
  - May not be set to the result of a function or any other value.
  - Set to a constant expressiong e.g. `const DATABASE_CONNECTION_NAME = 'connectionName';`

3. **Shadowing** - Repeated use of the keyword `let` for a variable.
``` rust
let x = 5;
let x = x + 1;
let x = x * 2;

// Output: The value of x is 12
println!("The value of x is: {}", x);

// Can be used to keep the same name for a variable even when types differ
// using `mut` in this manner will result in a compile time error
let spaces = "     ";
let spaces = spaces.len();
```

4. **Data Types**

**Static Types**: When converting types e.g. `string` -> `integer`, the rust compiler may need help.

``` rust
let guess = "42".parse().expect("Not A number!");
// ^^^^^^^^ cannot infer type

let guess: u32 = "42".parse().expect("Not A number!");
```

**Scalar Types**: Represent a single value: `integer`, `floating-point`, `Boolean`, and `characters`.

1. Integers - signed start with `i` and unsigned with `u`.
2. Floating Point -  `f32` and  `f64`
3. Boolean - `true` / `false`
4. Character - Character literals are with single quotes `'` and `string` literals are characterized with double quotes `"`.  Represents as unicode scalar values.
5. Compound Types
  - `tuple` represents multiple types in one group `let tup: (i32, i32) = (175, 36);`
  - retrieving values out of the tuple: `let (weight, waist) = tup;`
  - `array` - fixed length array of known values
  - `let a: [i32, 5] = [0, 15, 350, 200, 100];`


5. **Functions**
Keyword `fn` - parameters follow `name: type`.

- Statements and Expressions
  `statements` - do not return values `let x = 5;`
  `expressions` - returns a value, all functions are expressions.

- `Functions with return values` - `fn doStuff() -> i32 { }`
The final expression of a function is the return value.  You can return early by using the keyword `return`.

``` rust
fn early_return() -> 32 {
  return 12;
}

fn default_return() -> {
  13
}
```

6. **Comments**
- Used like javascript or typescript comments

``` rust
// This is comments, two forward slashes then your comment
```

7. **Control Flow**
- `if` doesn't evaluate to truthy or falsy values like JavaScript.  The value must evaluate to a Boolean.
- `if` is an expression that can evaluate to a `let` statement.
- `loop` - keyword for repeating a block of code
  - `break` keyword allows for breaking out of the loop
- `while` - conditional loop
- `for` - loop over a collection of items
  - `range` type provides a set of values over a ranges of values `(1..4)`
  - `rev()` - reverses the order of a collection
