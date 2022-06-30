// The syntax of the Straight-Line Program
pub mod syntax
{
    // Alias for strings
    type Id = String;

    // Binary operators
    enum BinOp
    {
        Plus,
        Minus,
        Times,
        Div
    }

    // Statements
    enum Stm
    {
        CompoundStm( Box<Stm>, Box<Stm> ),
        AssignStm( Id, Exp ),
        PrintStm( ExpList )
    }

    // Expressions
    enum Exp
    {
        IdExp( Id ),
        NumExp( i32 ),
        OpExp( Box<Exp>, BinOp, Box<Exp> ),
        EseqExp( Stm, Box<Exp> )
    }
}