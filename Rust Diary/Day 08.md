# Day 8
The interpreter finally works, praise Jesus.

Defensive copying is usually easy in Rust:

```rust 
fn foo( bar: &Bar )
{
	let baz = bar.clone();
	// etc.
}
```

Lexical scoping is good:

```rust

fn interpExp( exp: Exp, table: &Table ) -> (i32, Table)
{
    let new_table = table.clone();
    match exp
    {
		// etc.
        EseqExp( stm, exp )      => { interpExp( *exp, &interpStm( *stm, new_table ) )  }
    }
}
```

Once you get over the pointless complexity of borrowing, it's easy to emulate proper immutability and function purity.
