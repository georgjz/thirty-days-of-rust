# Day 1
Second try, I hate cargo.

Goals:
* ~~Installation~~
* ~~Hello World~~
* Simple, minimal library example


## Installation
Rustup offers you option `minimal/default/complete` for various items without telling you what it is - kinda pointless.

## First projects from book

>An _associated function_ is a function that’s implemented on a type, in this case `String`

That's a function.

Values in enumerations are called variants.

```rust
    io::stdin()

        .read_line( &mut guess )

        //.expect( "Failed to read line" );
```

Why does this result in a warning if the result of an expression(?) isn't used?

```
warning: unused variable: `secret_number`
 --> src\main.rs:7:9
  |
7 |     let secret_number = rand::thread_rng().gen_range( 1..101 );
  |         ^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_secret_number`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 2m 53s

```

Unused variables are a semantic action, shouldn't be encoded in syntax.

Range expressions `1..101`, where the lower boundry is inclusive, upper is exclusive. Equivalent to `1..=100`.

>A `match` expression is made up of _arms_. An arm consists of a _pattern_ to match against, and the code that should be run if the value given to `match` fits that arm’s pattern.

I think they're making up words.

```rust
		// weird
        let guess: u32 = match guess.trim().parse()

        {

            Ok( num ) => num,

            Err( _ )  => continue,

        }
```

no, not weird, missed the `match`.

Day 1 was rather frustrating; my dislike for package managers is clouding my judgement and experience. I have an obfuscated dependecy tree for simple RNG. `cargo tree` at least offers some transparency.

