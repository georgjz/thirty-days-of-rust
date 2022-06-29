# Day 3
Goals:
* Simple, minimal library example

## structs
Associate functions that specify structs' behavior are called methods. Structs and enums form the core of the type system.

The `unused variable` warning is kind of pointless; turning off that warning by prefixing `_` is a syntactical solution to a semantic problem.

```rust
struct User
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user( email: String, username: String ) -> User
{
	//User 
	//{
		//email: email,
		//username: username,
		//active: true,
		//sign_in_count: 1
	//}
	// can be shorthanded for fields with same name
	User 
	{
		email,
		username,
		active: true,
		sign_in_count: 1
	}
}

fn main()
{
    let mut user1 = User
    {
        email: String::from( "someone@example.com" ),
        username: String::from( "someusername123" ),
        active: true,
        sign_in_count: 1
    };
}
```

Why "type signature" for value assignment? struct update syntax is weird:

```rust 
let user2 = User
{
	email: String::from( "another@example.com" ),
	..user1
}
```

Mind `user1` was moved, so it is no longer valid.

### Tuple structs
Structs with anonymous fields in essence:

```rust
struct Color( i32, i32, i32 );
struct Point( i32, i32, i32 );

let black = Color( 0, 0, 0 );
let origin = Point( 0, 0, 0 );
```

Special precaution necessary to use references in structs.

```rust
struct Rectangle
{
    width: u32,
    height: u32
}

  

fn main()
{
    let rect1 = Rectangle { width: 30, height: 50 };
  

    println!( "The area of the rectangle is {} square pixels",
              area( &rect1 ) );
}
  

fn area( rect: &Rectangle ) -> u32
{
    rect.width * rect.height
}
```

Passing the reference here is kinda stupid, but necessary so `area` "borrows" `rect1`.

`dbg!` is almost useful, lol. Is it possible to redirect to logging file? This also encourages debugging in code. Single stepping a thing?

The above example rewritten with a struct method:

```rust
struct Rectangle
{
    width: u32,
    height: u32
}

impl Rectangle 
{
	fn area( &self ) -> u32
	{
		self.width * self.height
	}
}
 
fn main()
{
    let rect1 = Rectangle { width: 30, height: 50 };
  

    println!( "The area of the rectangle is {} square pixels",
              rect1.area() );
}
```

`&self` is shorthand for `self: &Self`. Functions inside a `impl` block are called *associated functions*. 

What's the rational behind using `.` for methods and `::` for associated functions? Seems related to modules/namespaces.
