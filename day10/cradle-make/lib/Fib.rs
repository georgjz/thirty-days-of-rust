pub fn fibonacci( n: u32 ) -> u64
{
    match n
    {
        0     => panic!(),
        1 | 2 => 1,
        _     => fibonacci( n - 1 ) + fibonacci( n - 2 )
    }
}