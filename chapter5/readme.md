# Chapter 5 - Structs to Structures Related Data

- `structs` are similar to other object oriented `structs` in which they group data together. These differ from `tuples` where in `tuples` you need to know the order of the values.

``` rust
struct User {
  first_name: String,
  last_name: String,
  email: String,
  sign_in_count: i32
}

// Defining and instance

let user = User {
  first_name: String::from("Howdy"),
  last_name: String::from("Doody"),
  email: String::from("Doody_Howdy@nowhere.com"),
  sign_in_count: 700
};

```

To access properties of the `struct` use the dot syntax.  If it's mutable use the `mut` keyword and assign a value to the particular property to be changed.

``` rust
let mut user = User {
  first_name: String::from("Howdy"),
  last_name: String::from("Doody"),
  email: String::from("Doody_Howdy@nowhere.com"),
  sign_in_count: 700
};

user.email = String::from("Chair Guy Zero");
```

When assign values to a new instance, there's a short hand way.  This involves keep the property names as the arguments.

``` rust 
fn build_user(email: String, first_name: String) -> User {
  User {
    email,
    first_name,
    sign_in_count: 0,
    last_name: String::from("unknown")
  }
}
```

Updating a struct give the instance properties of another is similar to JavaScript spread operator, but with two periods, e.g. `..user1_instance`.

``` rust
let user2 = User {
  email: String::from("hootie-hoo@yahoo.com"),
  ..user1
}
```

- `Tuple Structs` a struct that has an underlying tuple for values.

``` rust
struct RGBA(i32, i32, i32,i32);
struct Point(i32,i32,i32);

let color = RGBA(0,0,0,0);
let point = Point(2, 4, 5);
```

For debugging purposes of a struct formatting of a `struct` and be done within the `println!` macro using the syntax `{:?}`.  The `struct` definition must also be annotated with `#[derive(Debug)]`.  Using `{:#?}` will format the `structs` properties in a more readable format.

``` rust
#[derive(Debug)]
struct Rectangle {
  height: u32,
  width: u32,
};

println!("The rect properties {:?}", rect);
```

- **Methods** - similar to functions but they're defined in `struct` with the first parameter of `&self`.

``` rust
impl Rectange {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // method with parameters borrowing rectangle
    fn can_fit(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}
```

- **Associated Functions** - Functions that do not take `self` in the `impl` block but are associated with the struct.  For instance `String::from("New String!")`.  These are convienent for constructor methods.
