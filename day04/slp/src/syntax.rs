// The syntax of the Straight-Line Program

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
    PrintStm( ExpList )
}

// Expressions
pub enum Exp
{
    IdExp( Id ),
    NumExp( i32 ),
    OpExp( Box<Exp>, BinOp, Box<Exp> ),
    EseqExp( Box<Stm>, Box<Exp> )
}

pub type ExpList = Vec<Exp>;
