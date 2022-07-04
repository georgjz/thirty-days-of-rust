#![feature(box_patterns)] // this feature is not stable yet

mod syntax;
mod table;

use crate::syntax::*;
use crate::syntax::BinOp::*;
use crate::syntax::Stm::*;
use crate::syntax::Exp::*;
use crate::table::*;

fn maxargs( stm: &Stm ) -> i32
{
    match stm
    {
        CompoundStm( stm1, stm2 )              => { std::cmp::max( maxargs( &*stm1 ), maxargs( &*stm2 ) ) },
        AssignStm( _, box EseqExp( stm, _ ) )  => { maxargs( &*stm ) },
        AssignStm( _, _ )                      => { 0 },
        PrintStm( exps )                       => { exps.len() as i32 }
    }
}

// Actual interpreter functions
fn interp( stm: Stm )
{
    interpStm( stm, Table::new() );
}

fn interpStm( stm: Stm, table: Table ) -> Table
{
    match stm
    {
        CompoundStm( stm1, stm2 )  =>
        {
            interpStm( *stm2, interpStm( *stm1, table ) )
        },

        AssignStm( id, box exp )   =>
        {
            let (new_value, exp_table) = interpExp( exp, &table );
            let new_table = update( exp_table, id, new_value );
            new_table
        },

        PrintStm( exps )  =>
        {
            let mut new_table = table.clone();
            for exp in exps
            {
                let (value, foo) = interpExp( exp, &new_table );
                new_table = foo;
                print!( "{} ", value );
            }
            // exps.iter().map( |exp| println!("{} ", interpExp( exp, &table ).0 ) );
            println!( "" );
            new_table
        }
    }
}

fn interpExp( exp: Exp, table: &Table ) -> (i32, Table)
{
    let new_table = table.clone();
    match exp
    {
        IdExp( id )              => { (lookup( &table, id ), new_table) },
        NumExp( value )          => { (value, new_table) },
        OpExp( lhs, Plus, rhs )  => { (interpExp( *lhs, &new_table ).0 + interpExp( *rhs, &new_table ).0, new_table) },
        OpExp( lhs, Minus, rhs ) => { (interpExp( *lhs, &new_table ).0 - interpExp( *rhs, &new_table ).0, new_table) },
        OpExp( lhs, Times, rhs ) => { (interpExp( *lhs, &new_table ).0 * interpExp( *rhs, &new_table ).0, new_table) },
        OpExp( lhs, Div, rhs )   => { (interpExp( *lhs, &new_table ).0 / interpExp( *rhs, &new_table ).0, new_table) },
        EseqExp( stm, exp )      => { interpExp( *exp, &interpStm( *stm, new_table ) )  }
    }
}

// main
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

    println!( "margs( prog ): {}", maxargs( &prog ) );

    interp( prog );

    println!( "Hail Satan" );
}
