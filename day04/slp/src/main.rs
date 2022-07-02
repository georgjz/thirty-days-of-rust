mod syntax;
use crate::syntax::*;
use crate::syntax::BinOp::*;
use crate::syntax::Stm::*;
use crate::syntax::Exp::*;

fn main()
{
    // The test program to run
    let prog : Stm =
        CompoundStm(
         // a := 5 + 3
         Box::new( AssignStm( "a".to_string(), Box::new( OpExp( Box::new( NumExp( 5 ) ), Plus, Box::new( NumExp( 3 ) ) ) ) ) ),
         Box::new( CompoundStm(
                    // b := (print( a, a - 1), 10 * a)
                    Box::new( AssignStm( "b".to_string(), Box::new( EseqExp( Box::new( PrintStm( vec![ IdExp( "a".to_string() )
                                                                                                     , OpExp( Box::new( IdExp( "a".to_string() ) ), Minus, Box::new( NumExp( 1 ) ) ) ] ) ),
                                                                 Box::new( OpExp( Box::new( NumExp( 10 ) ), Times, Box::new( IdExp( "a".to_string() ) ) ) ) ) ) ) ),
                    // print( b )
                    Box::new( PrintStm( vec![ IdExp( "b".to_string() ) ] ) ) ) ) );
        //  )
        // );

    // let foo = PrintStm( Box::new( vec![ IdExp( "a".to_string() ) ] ) );
    let foo = PrintStm( vec![ IdExp( "a".to_string() ) ] );

    println!( "Hail Satan" );
}
