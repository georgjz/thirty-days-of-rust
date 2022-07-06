#![crate_name="fibonacci"]
#![crate_type = "lib"]

pub fn fibonacci( n: u32 ) -> u64
{
    match n
    {
        0..=2 => 1,
        3     => 2,
        _     => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main()
{
    println!( "{}", fibonacci( 10 ) );
}
