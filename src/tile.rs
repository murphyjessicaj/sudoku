//
// implemnts tiles of sudoku game board
//

pub struct Tile {
    number: i32
}

impl Tile {
    pub fn new() -> Tile {
        let mut tile = Tile {
            number: 0
        };
        tile
    }
}
