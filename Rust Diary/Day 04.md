# Day 4
Goals:
* Simple, minimal library example

## Enums
Like ADT, allegedly. As in, "we can enumerate all possibilities". Enums are namespaced under the identifier

```rust 
enum IpAddrKind 
{
	V4,
	V6
}

let four = IpAddrKind::V4;
let six  = IpAddrKind::V6;
```

Each variant is a value constructor:

```rust
enum IpAddr 
{
	V4( String ),
	V6( String )
}
```

Removing `null` fixes all problems, apparently. Just citing Tony Hoare doesn't make it right. 

Type constructors are a thing:

```rust 
enum Option<T> 
{
	None,
	Some( T )
}
```

## Pattern matching
```rust
enum Coin 
{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 
{
    match coin 
    {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
```

Lets do a Straight-Line Program.

## SLP
Taken from *Modern Compiler Implementation in ML*, Appel et. al.

`mod` = `#include` - LOL

How to recursive types work in Rust?