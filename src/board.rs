//
// Implements the main game logic
//

pub struct Board {
    tiles: [[i32; 9]; 9],
    boxes: [[i32; 3]; 3],
}


impl Board {
    pub fn new() -> Board {
        let mut board = Board{
            tiles: [
                [5,3,0,0,7,0,0,0,0],
                [6,0,0,1,9,5,0,0,0],
                [0,9,8,0,0,0,0,6,0],
                [8,0,0,0,6,0,0,0,3],
                [4,0,0,8,0,3,0,0,1],
                [7,0,0,0,2,0,0,0,6],
                [0,6,0,0,0,0,2,8,0],
                [0,0,0,4,1,9,0,0,5],
                [0,0,0,0,8,0,0,7,9]
            ],
            //win tile
            //tiles: [
            //    [5,3,4,6,7,8,9,1,2],
            //    [6,7,2,1,9,5,3,4,8],
            //    [1,9,8,3,4,2,5,6,7],
            //    [8,5,9,7,6,1,4,2,3],
            //    [4,2,6,8,5,3,7,9,1],
            //    [7,1,3,9,2,4,8,5,6],
            //    [9,6,1,5,3,7,2,8,4],
            //    [2,8,7,4,1,9,6,3,5],
            //    [3,4,5,2,8,6,1,7,9]
            //],
            boxes: [
                [0,1,2],
                [3,4,5],
                [6,7,8]
            ],
        };
        board
    }

    pub fn update(&mut self) {
        //check if won

        //check columns

        let mut won: bool = true;

        for i in 0..9 {
            let mut check = vec![0; 9];
            for j in 0..9 {
                if self.tiles[i][j]==0 {
                    won = false;
                    break;
                } else {
                    check[(self.tiles[i][j]-1) as usize]+=1;
                }
            }
            for j in check {
                if j==0 || j>1 {
                    won = false;
                    break;
                }
            }
        }

        //check rows

        for i in 0..9 {
            let mut check = vec![0; 9];
            for j in 0..9 {
                if self.tiles[j][i]==0 {
                    won = false;
                    break;
                } else {
                    check[(self.tiles[j][i]-1) as usize]+=1;
                }
            }
            for j in check {
                if j==0 || j>1 {
                    won = false;
                    break;
                }
            }
        }

        //check boxes
        for i in 0..3 {
            for j in 0..3 {
                let mut check = vec![0; 9];

                for ii in &self.boxes[i] {
                    for jj in &self.boxes[j] {
                        let num = self.tiles[(*ii) as usize][(*jj) as usize];
                        if num==0 {
                            won = false;
                            break;
                        } else {
                            check[(num-1) as usize] += 1;
                        }
                        //println!("{},{}",ii,jj);
                    }
                }

                for k in check {
                    if k==0 || k>1 {
                        won = false;
                        break;
                    }
                }
            }
        }


        if won {
            println!("You won!");
        }

    }
}
