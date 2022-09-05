# Day 1
Third try, I hate cargo.

Goals:
* ~~Installation~~
* ~~Hello World~~
* Simple, minimal library example

Downsides as of section 1.8:
* cyclic datastructures: that's a bit lame, but lets see
* compile time: a made up problem
* strictness: laziness is fun for concise code expressions, but not a deal breaker

## 2 Language Foundations

accessing containers like in
```rust
for item in collection
```

will invalidade the `collection`, because ownership.

loop labels are a thing in Rust:
```rust
'outer for x in 0..
{
    // bla
    for z in 0..
    {
        // blash
        break 'outer;
    }
}
```

Loop labels must be prefixed with apostrophe `'`.

Pattern matching

```rust
match item
{
    0         => {},
    10 ..= 20 => {},
    40 | 80   => {},
    _         => {}
}
```

defining functions 

```rust
fn add( i: i32, j: i32 ) -> i32
{
    i + j
}
```

### 2.6 References
Create references with the reference operator `&`, dereference with `*`:
```rust
fn main()
{
    let a = 42;
    let r = &a;
    let b = a + *r;
    println!( "a + a = {}", b ); 
}
```

Iterate over reference to collection to preserve it 
```rust
for item in &collection
{
    if *item { ... } // element must be dereferences
}
```

`cargo add num` adds the `num` package as project dependency.

continue with mandelbrot project.
