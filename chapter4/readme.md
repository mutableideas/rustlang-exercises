# Chapter 4 - Understanding Ownership

## Ownership Rules
- Each value in Rust has a variable that's called it's owner
- when the owner goes out of scope, the value will be dropped.

- Problem with memory allocation is that via the Stack (Last In First Out) and Heap (dynamic sizing and searching for open memory and pointers to that space)
- Rust takes the approach of freeing memory from the heap once a variable goes out of scope.  Garbage Collectors, like in C#, do this automatically via best guess if called directly.
- At the closing curly brace, Rust uses it's own `drop` to remove the item from memory.  This happens at the closing brace of the method.

- Rust when copying variables to one another does a shallow copy in which it createa pointer to the same memory.  There is no automatic deep copying.
- Copying deeply involves using a `clone()` method.

``` rust
let s1 = String::from("howdy");

// Now has it's own deep copy of s1
let s2 = s1.clone();
```

The following is valid, because integers are of a known size and can come off the stack easily.

``` rust
let x = 5;
let y = x;
```

- Ownership is the concept of a function taking a variable into it's scope and owning.  This makes the function responsible for dropping a value on the heap.

``` rust

fn do_stuff() {
  let s1 = String::from("Hootie Hoo!");
  let s2 = take_ownership(s1);
}

// Takes and gives back ownership to the calling function
fn take_ownership(val: String) -> String {
  return val;
}

```

- **References and Borrowin** - the `&` symbol allows to reference / borrow a variable from another function's scope for use.

``` rust
let s1 = String::from("Hootie Hoo!");
let size = reference_borrowing(&s1);

fn reference_borrowing(val: &String) -> usize {
    val.len()
}
```

- References are `immutable` and will throw errors when trying to change it's value.

- **Mutable References** - Allows a function to mutate the data for a given variable, denoted by `&mut`.
- You may only have one mutable reference to particular piece of data in scope.

``` rust
// This doesn't work
let s1 = String::from("Hello");
let r1 = &mut s1;
let r2 = &mut s2;
```

- **Dangling References** Pointer that references a localtion in memory that may have been given to someone else by freeing some memory while preserving the pointer.  Rust compiler helps prevent these.

``` rust
let hootie = dangle();

fn dangle() -> &String {
  let s1 = String::from("hi!");
  &s1
} // s1 goes out of scope and memory is cleared
```

- **Rules of References**
  - You have have either one mutable reference or any number of immutable references
  - References must always be valid

- **Slice Type**
  - Does not have ownership
  - References a contiguous squesnce of elements in a collection
  - `Problem` - the `fn first_word` method returns an index at a particular state of the string.

- **String Slices**
  - Slicing a sring `&string_var[start_index..end_index]` or `&word[0..3]`
  - Represented as a type using `&str`.

- **Other Slices**
  - Slicing an `i32`

``` rust
let a = [1,2,3,4,5];
let slice = &a[1..3] // [2, 3, 4]
```



