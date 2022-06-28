# Day 2
Goals:
* Simple, minimal library example

## Serfdom / Ownership
* Values are owned by variables, the *owner*
* One owner only
* Owner goes out of scope, value dropped

`drop` == `delete` (bite me)

Rust does not deep copy, instead, `s1` is *moved* into `s2`:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```

`s1` is an invalid reference after it's moved into `s2`. To create deep copies, use `clone`:

```rust
let s1 = String::from("hello"); 
let s2 = s1.clone(); 

println!("s1 = {}, s2 = {}", s1, s2);
```

Traits seem in essence to be typeclasses. The `Copy` trait allows variables to remain valid after assignment.

```rust
fn main() 
{
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) 
{ // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) 
{ // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

This seems an odd choice; does this lead to unexpected behavior? Loss of data?

Use *references* to use values without taking ownership. Creating a reference is called *borrowing*. **NB!** need to be explicit when passing references:

```rust 
fn main() 
{
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // NB!

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize 
{
    s.len()
}
```

Mutable references `&mut` allow values to be modified. **NB!** there can only be one mutable reference to any given variable in scope at any given time. Multiple immutable references are allowed, but no mutable and immutable reference can be in the same scope.

```rust
fn main() 
{
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

We can use *slices*: 

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

hooray. Slices seem utterly pointless, lel.