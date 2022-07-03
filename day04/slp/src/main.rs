#![feature(box_patterns)] // this feature is not stable yet

mod syntax;
mod table;

use crate::syntax::*;
use crate::syntax::BinOp::*;
use crate::syntax::Stm::*;
use crate::syntax::Exp::*;
use crate::table::*;

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

// Actual interpreter functions
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
            for exp in exps
            {
                let (value, _) = interpExp( exp, &table );
                println!( "{} ", value );
            }
            // exps.iter().map( |exp| println!("{} ", interpExp( exp, &table ).0 ) );
            println!( "" );
            table
        }
    }
}

fn interpExp( exp: Exp, table: &Table ) -> (i32, Table)
{
    // let new_table = Table::new();
    match exp
    {
        IdExp( id ) => { let table_copy = *table; (lookup( &table, id ), table_copy) },
        IdExp( id ) => { (lookup( &table, id ), table_copy) },
        _ => (666, *table)
    }
    // (666, new_table)
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

    println!( "margs( prog ): {}", maxargs( prog ) );

    // test table
    let mut table = Table::new();
    println!( "Number of entries: {}", table.len() );
    table = update( table, "a".to_string(), 5 );
    println!( "Number of entries: {}", table.len() );
    println!( "Value of a in symbol table: {}", lookup( &table, "a".to_string() ) );


    println!( "Hail Satan" );
}
