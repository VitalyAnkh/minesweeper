use rand::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
enum VisibleElement {
    Square(&'static str),
    Number(u32),
}
impl fmt::Display for VisibleElement{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        match self{
            VisibleElement::Square(s)=>{
                write!(f,"{}",s)
            },
            VisibleElement::Number(n)=>{
                write!(f,"{}",n)
            }
        }
    }
}


#[derive(Debug, Clone)]
struct Square {
    has_mine: bool,
    neighbor_mine_num: u32,
    visible: VisibleElement,

}
const DIAMOND_HOLLOW:&str="▢";
const DIAMOND_FLOWERY:&str="▦";
const DIAMOND_WHITE_LARGE:&str="⬜";
const DIAMOND_WHITE_MEDIUM:&str="◻";
const BOMB:&str="💣";
//static DIAMOND_VEC: Vec<u8> = vec![0xE2, 0x96, 0xA1];
//static DIAMOND: String = String::from_utf8(DIAMOND_VEC).unwrap();
//static BOMB_VEC: Vec<u8> = vec![0xF0, 0x9F, 0x92, 0xA3];
//static BOMB: String = String::from_utf8(BOMB_VEC).unwrap();

#[derive(Debug)]
struct Grid(Vec<Vec<Square>>);

impl Grid {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
//        v.shuffle(&mut rng);
        let mut grid = Grid(vec![vec![Square { neighbor_mine_num: 0, visible: VisibleElement::Square(DIAMOND_WHITE_LARGE), has_mine: false }; 50]; 50]);
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

    fn display(&self) {
        for i in 1..self.0[0].len(){
            print!("{} ",i);
        }
        for x in &self.0{
            println!("");
            for y in x{
                print!("{} ",y.visible);
            }
        }
    }
    fn check_valid() {}
}

fn main() {
    println!("Hello, Mine Sweeper!");
    let mut grid = Grid::new();

    println!("{}{}{}", DIAMOND_FLOWERY, DIAMOND_HOLLOW, DIAMOND_FLOWERY);

    println!("{}{}{}", DIAMOND_HOLLOW, DIAMOND_FLOWERY, DIAMOND_HOLLOW);
    println!("{}{}{}",DIAMOND_WHITE_LARGE,DIAMOND_WHITE_LARGE,DIAMOND_WHITE_LARGE);
    println!("{}{}{}",DIAMOND_WHITE_MEDIUM,DIAMOND_WHITE_MEDIUM,DIAMOND_WHITE_MEDIUM);
    println!("{}{}{}",1,2,3);
    grid.display();

}