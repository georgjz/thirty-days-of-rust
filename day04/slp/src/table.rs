// Symbol table implemented as a HashMap
use std::collections::HashMap;
use crate::syntax::Id;

pub type Table = HashMap<String, i32>;

pub fn update( mut table: Table, key: Id, value: i32 ) -> Table
{
    // let mut new_table : Table = table;
    // if let Some( new_table ) = table.insert( key, value ) { new_table }
    table.insert( key, value );
    table
    // panic!( "Couldn't update symbol table" );
}

pub fn lookup( table: &Table, key: Id ) -> i32
{
    if let Some( val ) = table.get( &key ) { return *val; }
    panic!( "Key not in symbol table" );

    // match table.get( &key )
    // {
    //     Some( val ) => *val,
    // }
}
