mod syntax;
// use crate::syntax;

fn main()
{
    // The test program to run
    // let prog : Stm =
    //     CompoundStm(
    //      // a := 5 + 3
    //      box AssignStm( "a", box OpExp( NumExp( 5 ), Plus, NumExp( 3 ) ) ),
    //      box CompoundStm(
    //           // b := (print( a, a - 1), 10 * a)
    //           box AssignStm( "b", box EseqExp( box PrintStm( list![ IdExp( "a" )
    //                                                               , OpExp( box IdExp( "a" ), Minus, box NumExp( 1 ) ) ]),
    //                                            box OpExp( box NumExp( 10 ), Times, box IdExp( "a" ) ) ) ),
    //           // print( b )
    //           box PrintStm( list![IdExp( "b" )] )
    //      )
    //     );

    let foo = syntax::Stm::PrintStm( list![ syntax::Exp::IdExp( "a" ) ] );

    println!( "Hail Satan" );
}
