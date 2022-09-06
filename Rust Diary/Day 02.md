# Day 2
Generic functions
```rust
fn add<T>( i: T, j: T ) -> T 
{

}
```

Traits (which are typeclass/concept) are bound to a function thusly
```rust
fn add<T: std::ops::Add<Output = T>>( i: T, j: T ) -> T
{
    i + j
}
```

`T` must implement `std::ops::Add` trait for this function to compile correctly. **All Rust operators are syntactic sugar for a trait's methods.** Aka, `a + b` `==` `a.add( b )`

Reading chapter 2 was pain.

## 3 Compound Data Types
The good stuff, finally (I hope). 
```rust
struct File 
{
    name: String,
    data: Vec<u8>
}
```

implementing "default constructor"
```rust
impl File 
{
    fn new( name: &str ) -> File 
    {
        File
        {
            name: string::from( "name" ),
            data: Vec::new()
        }
    }
}
```

```rust
trait Read 
{
    fn read( self: &Self, save_to: &mut Vec<u8> ) -> Result<usize, String>;
}
```

