mod syntax;
use crate::syntax::*;
use crate::syntax::BinOp::*;
use crate::syntax::Stm::*;
use crate::syntax::Exp::*;

// fn maxargs( stm: Stm ) -> i32
// {
//     match stm
//     {
//         CompoundStm( stm1, stm2 )           => { std::cmp::max( maxargs( *stm1 ), maxargs( *stm2 ) ) },
//         AssignStm( _, Box<EseqExp( stm, _ )> )  => { maxargs( stm ) },
//         AssignStm( _, _ )                   => { 0 },
//         PrintStm( exps )                    => { i32::from( exps.len() ) }
//     }
// }

fn main()
{
    // The test program to run
    let prog : Stm =
        CompoundStm(
         // a := 5 + 3
         AssignStm( "a".to_string(), OpExp( NumExp( 5 ), Plus, NumExp( 3 ) ) ),
         CompoundStm(
          // b := (print( a, a - 1), 10 * a)
          AssignStm( "b".to_string(), EseqExp( PrintStm( vec![ IdExp( "a".to_string() )
                                                             , OpExp( IdExp( "a".to_string() ), Minus, NumExp( 1 ) ) ] ),
                                               OpExp( NumExp( 10 ), Times, IdExp( "a".to_string() ) ) ) ),
          // print( b )
          PrintStm( vec![ IdExp( "b".to_string() ) ] ) ) );
        //  )
        // );

    // let foo = PrintStm( Box::new( vec![ IdExp( "a".to_string() ) ] ) );
    let foo = PrintStm( vec![ IdExp( "a".to_string() ) ] );

    println!( "{}", maxargs( prog ) );
    println!( "Hail Satan" );
}
