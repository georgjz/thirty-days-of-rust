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
        CompoundStm( &Stm, &Stm ),
        AssignStm( Id, &Exp ),
        // PrintStm( Box<ExpList> )
        PrintStm( ExpList )
    }

    // Expressions
    pub enum Exp
    {
        IdExp( Id ),
        NumExp( i32 ),
        OpExp( &Exp, BinOp, &Exp ),
        EseqExp( &Stm, &Exp )
    }

    // FIXME: This compiles, but can't be called for some reason
    // Expression list, lets play with ADT
    pub enum List<A>
    {
        Nil,
        Cons( A, Box<List<A>> )
    }

    // pub type ExpList = List<Exp>;
    pub type ExpList = Vec<Exp>;

    // Macro to make life easier
    #[macro_export] // why?
    macro_rules! list[
        ()                         => ( Nil );
        ($x:expr)                  => ( Cons ( $x, box Nil ) );
        ($x:expr, $($xs:expr), + ) => ( Cons( $x, box list!( $( $xs ), + )));
    ];
// }