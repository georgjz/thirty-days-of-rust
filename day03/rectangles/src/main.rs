#[derive(Debug)]
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
    println!( "Rect1: {:?}", rect1 );

    // this is useful
    dbg!( 30 * 40 );
}

fn area( rect: &Rectangle ) -> u32
{
    rect.width * rect.height
}
