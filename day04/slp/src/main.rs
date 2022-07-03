#![feature(box_patterns)] // this feature is not stable yet

mod syntax;
mod table;

use crate::syntax::*;
use crate::syntax::BinOp::*;
use crate::syntax::Stm::*;
use crate::syntax::Exp::*;

fn maxargs( stm: Stm ) -> i32
{
    match stm
    {
        CompoundStm( stm1, stm2 )              => { std::cmp::max( maxargs( *stm1 ), maxargs( *stm2 ) ) },
        AssignStm( _, box EseqExp( stm, _ ) )  => { maxargs( *stm ) },
        AssignStm( _, _ )                      => { 0 },
        PrintStm( exps )                       => { exps.len() as i32 }
    }
}

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

    println!( "margs( prog ): {}", maxargs( prog ) );
    println!( "Hail Satan" );
}
