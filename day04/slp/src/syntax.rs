// The syntax of the Straight-Line Program
// pub mod syntax
// {
    // Alias for strings
    pub type Id = String;

    // Binary operators
    pub enum BinOp
    {
        Plus,
        Minus,
        Times,
        Div
    }

    // Statements
    pub enum Stm
    {
        CompoundStm( Box<Stm>, Box<Stm> ),
        AssignStm( Id, Box<Exp> ),
        PrintStm( Box<ExpList> )
    }

    // Expressions
    pub enum Exp
    {
        IdExp( Id ),
        NumExp( i32 ),
        OpExp( Box<Exp>, BinOp, Box<Exp> ),
        EseqExp( Box<Stm>, Box<Exp> )
    }

    // Expression list, lets play with ADT
    pub enum List<A>
    {
        Nil,
        Cons( A, Box<List<A>> )
    }

    pub type ExpList = List<Exp>;

    // Macro to make life easier
    #[macro_export] // why?
    macro_rules! list[
        ()                         => ( Nil );
        ($x:expr)                  => ( Cons ( $x, box Nil ) );
        ($x:expr, $($xs:expr), + ) => ( Cons( $x, box list!( $( $xs ), + )));
    ];
// }