fn greet_world()
{
    println!("Hello, world!");
    let southern_german = "Grüss Gott!";
    let japan = "こにちは";
    let regions = [ southern_german, japan ];
    for region in regions
    {
        println!( "{}", region );
    }
}


fn main()
{
    greet_world();
}
