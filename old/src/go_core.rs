#![crate_name="go_core"]

pub mod go {
	use std::fmt;

	pub enum Stone {
		Black,
		White,
	}

	pub type Direction = ( i8, i8);
	pub type Position = ( u8, u8);

	pub struct Cell {
		stone: Option<Stone>
	}

	impl Cell {
		pub fn new() -> Cell {
			Cell{ stone: None}}
	}

	pub type Grid = Vec< Vec<Cell>>;

	pub struct Board {
		dim : u8,
		grid : Grid,
	}

	impl Board {
		//normal constructor
		pub fn new() -> Board {
			Board::with_dim( 17)}

		//sized constructor
		pub fn with_dim( dim : u8) -> Board {
			//grid builder closure
			let grid_builder = | uint | -> Vec<Cell> {
				Vec::from_fn( dim as uint,
					//row builder closure
					| uint | -> Cell { Cell::new()})};

			//make grid
			let grid : Grid =
				Vec::from_fn( dim as uint, grid_builder);

			//make board
			Board{ dim: dim, grid: grid}}

		pub fn get( &self, x : Position){}
		pub fn put( &mut self, x : Position, stone : Stone){}
		pub fn remove( &mut self, x : Position){}
		//check if a move is valid
		pub fn check( &self, x : Position, stone : Stone){}
		//place a move
		pub fn place( &mut self, x : Position, stone : Stone){}
	}

	impl fmt::Show for Board {
		fn fmt( &self, formatter: &mut fmt::Formatter)
				-> fmt::Result {
			write!( formatter, "Go board says hi :)")}
}
}
