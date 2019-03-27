use rand::prelude::*;

#[derive(Debug)]
enum VisibleElement{
    Square(String),
    Number(u32)
}
#[derive(Debug, Copy, Clone)]
struct Square {
    has_mine: bool,
    neighbor_mine_num: u32,
    visible: VisibleElement,

}


#[derive(Debug)]
struct Grid(Vec<Vec<Square>>);

impl Grid {
    fn new() -> Self {
        let s:Vec<u8>=vec![0xE2,0x96,0xA1];
        let s=String::from_utf8(s).unwrap();
        let mut rng = rand::thread_rng();
//        v.shuffle(&mut rng);
        let mut grid = Grid(vec![vec![Square { neighbor_mine_num: 0, visible: VisibleElement::Square(s), has_mine: false }; 100]; 100]);
//        let mut v: Vec<Vec<Square>> = Vec::new();
        for i in 0..grid.0.len() {
            for j in 0..grid.0[1].len() {
                grid.0[i][j].has_mine = rng.gen::<bool>();
            }
        }

//        tmp.shuffle(&mut rng);
        grid
    }

    /// As a beginning, to greet player and prompt the player to input coordinates
    fn begin(&mut self) {}

    fn display(&self) {}
    fn check_valid() {}
}

fn main() {
    println!("Hello, Mine Sweeper!");
    let mut grid = Grid::new();
    let s:Vec<u8>=vec![0xE2,0x96,0xA1];
    let s=String::from_utf8(s).unwrap();
    println!("{}{}{}",s,s,s);
    let m:Vec<u16>=vec![0x25A9];
    let m=String::from_utf16(&m).unwrap();
    println!("{}",m);
}