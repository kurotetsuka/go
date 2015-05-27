#![crate_name="go_game"]

//local imports
extern crate go_core;
use go_core::go;

fn main(){
	let board = go::Board::new();
	println!("hello :)");}
