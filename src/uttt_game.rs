use std::fmt::{Display, Debug};

#[derive(PartialEq)]
enum Mark {
    Empty,
    X,
    O,
}

pub enum BoardError {
    MoveAtNotSentBoard,
    MoveAtAlreadyFilledTile,
    MoveAtAlreadyFinishedBoard,
}

impl Debug for BoardError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MoveAtNotSentBoard => write!(f, "MoveAtNotSentBoard"),
            Self::MoveAtAlreadyFilledTile => write!(f, "MoveAtAlreadyFilledTile"),
            Self::MoveAtAlreadyFinishedBoard => write!(f, "MoveAtAlreadyFinishedBoard"),
        }
    }
}

#[derive(PartialEq)]
pub enum Player {
    X,
    O,
}
impl Player {
    const fn to_mark(&self) -> Mark {
        match self {
            Player::X => Mark::X,
            Player::O => Mark::O,
        }
    }
    pub const fn get_letter(&self) -> &str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }
}


#[derive(Debug)]
pub enum WonByPlayer {
    X,
    O,
    Tie,
    HasntFinished,
}

struct Tile {
    //big_board_index:u8,
    //small_board_index:u8,
    mark: Mark,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_letter())
    }
}

impl Tile {
    const fn new() -> Self {
        Tile { mark: Mark::Empty }
    }
    const fn get_letter(&self) -> &str {
        match self.mark {
            Mark::Empty => " ",
            Mark::X => "X",
            Mark::O => "O",
        }
    }
}

struct SmallBoard {
    tiles: [Tile; 9],
    x_bits: u16,
    o_bits: u16,
    filled_tiles: u8,
    won_by: WonByPlayer,
}

impl Display for SmallBoard {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self.won_by{
            WonByPlayer::X => write!(f, "{}\n{}\n{}\n{}\n{}", SmallBoard::WON_BY_X_BOARD_SHAPE[0], SmallBoard::small_horizontal_seperator(), SmallBoard::WON_BY_X_BOARD_SHAPE[1], SmallBoard::small_horizontal_seperator(), SmallBoard::WON_BY_X_BOARD_SHAPE[2]),
            WonByPlayer::O => write!(f, "{}\n{}\n{}\n{}\n{}", SmallBoard::WON_BY_O_BOARD_SHAPE[0], SmallBoard::small_horizontal_seperator(), SmallBoard::WON_BY_O_BOARD_SHAPE[1], SmallBoard::small_horizontal_seperator(), SmallBoard::WON_BY_O_BOARD_SHAPE[2]),
            WonByPlayer::Tie => write!(f, "{}\n{}\n{}\n{}\n{}", SmallBoard::TIE_BOARD_SHAPE[0], SmallBoard::small_horizontal_seperator(), SmallBoard::TIE_BOARD_SHAPE[1], SmallBoard::small_horizontal_seperator(), SmallBoard::TIE_BOARD_SHAPE[2]),
            WonByPlayer::HasntFinished => write!(
                f,
                " {} | {} | {} \n\
                 ---+---+---\n\
              \x20{} | {} | {} \n\
                 ---+---+---\n\
              \x20{} | {} | {} ",
                self.tiles[0],
                self.tiles[1],
                self.tiles[2],
                self.tiles[3],
                self.tiles[4],
                self.tiles[5],
                self.tiles[6],
                self.tiles[7],
                matches!(self.won_by, WonByPlayer::HasntFinished)
            ),
        }
        
    }
}

impl SmallBoard {
    /*fn get_board(&self) -> String {
        format!(
            "{} {}
        ",
            self.tiles[0]
        )
    } */
    
    const fn new() -> Self {
        const INIT: Tile = Tile::new();
        Self {
            tiles: [INIT; 9],
            filled_tiles: 0,
            won_by: WonByPlayer::HasntFinished,
            x_bits: 0,
            o_bits: 0,
        }
    }
    
    const WON_BY_O_BOARD_SHAPE: [&'static str;3] =
        [" / | - | \\ ",
         " | |   | | ", 
         " \\ | - | / "];
    

    const WON_BY_X_BOARD_SHAPE:  [&'static str;3] =
        [" \\ |   | / ", 
         "   | X |   ",
         " / |   | \\ "];

    const TIE_BOARD_SHAPE: [&'static str;3] =
       [" \\ | - | / ",
        " | | # | | ",
        " / | - | \\ "];
    
    /*fn get_board(&self) -> String {
        // is it faster to create string in-memory and then print or should i print! on every char?
        format!("{}", self)
    }*/
    fn get_row(&self, row: u8) -> String {
        assert!(row <= 2);
        match self.won_by {
            WonByPlayer::X => SmallBoard::WON_BY_X_BOARD_SHAPE[usize::from(row)].to_owned(),
            WonByPlayer::O => SmallBoard::WON_BY_O_BOARD_SHAPE[usize::from(row)].to_owned(),
            WonByPlayer::Tie => SmallBoard::TIE_BOARD_SHAPE[usize::from(row)].to_owned(),
            WonByPlayer::HasntFinished =>format!(
                " {} | {} | {} ",
                self.tiles[3*usize::from(row)],
                self.tiles[3*usize::from(row) + 1],
                self.tiles[3*usize::from(row) + 2]
            ),
        }

    }
    fn place_tile_and_record_finish(&mut self, position: u8, player: &Player) -> Result<&WonByPlayer, BoardError> {
        if self.tiles[usize::from(position)].mark != Mark::Empty {
            return Err(BoardError::MoveAtAlreadyFilledTile); // Move at already filled tile
        }

        //xoring for presumably better speed
        //(cant unset a bit as we check above for bit not being set)
        match player {
            Player::X => self.x_bits ^= 1 << position,
            Player::O => self.o_bits ^= 1 << position,
        }

        self.filled_tiles+=1;

        self.tiles[usize::from(position)].mark = player.to_mark();
        self.check_finish_and_record();
        return Ok(&self.won_by);
    }

    fn check_draw(&self) -> bool {
        if self.filled_tiles == 9 {
            return true;
        } else {
            return false;
        }
    }

    const fn small_horizontal_seperator() -> &'static str{
        "---+---+---"
    }

    const fn horizontal_seperator() -> &'static str{
        "---+---+---║---+---+---║---+---+---"
    }

    fn check_win(&self) -> WonByPlayer {
        // since these values are being accessed more than once it would
        // probably be a better idea to cache them or use a different structure
        // and then apply 2 bitmasks to check if either X or O has won.

        // on that now, should probably get git to work by now

        if self.x_bits & 0b001001001 == 0b001001001 // vertical
            || self.x_bits & 0b010010010 == 0b010010010
            || self.x_bits & 0b100100100 == 0b100100100
            || self.x_bits & 0b000000111 == 0b000000111 //horizontal
            || self.x_bits & 0b000111000 == 0b000111000
            || self.x_bits & 0b111000000 == 0b111000000
            || self.x_bits & 0b100010001 == 0b100010001 //diagonal
            || self.x_bits & 0b001010100 == 0b001010100
        {
            return WonByPlayer::X;
        } else if self.o_bits & 0b001001001 == 0b001001001 //vertical
            || self.o_bits & 0b010010010 == 0b010010010
            || self.o_bits & 0b100100100 == 0b100100100
            || self.o_bits & 0b000000111 == 0b000000111 //horizontal
            || self.o_bits & 0b000111000 == 0b000111000
            || self.o_bits & 0b111000000 == 0b111000000
            || self.o_bits & 0b100010001 == 0b100010001 //diagonal
            || self.o_bits & 0b001010100 == 0b001010100
        {
            return WonByPlayer::O;
        }
        return WonByPlayer::HasntFinished;
    }

    fn get_current_finish_status(&self) -> WonByPlayer {
        if self.check_draw() {
            return WonByPlayer::Tie;
        }
        return self.check_win();
    }

    fn check_finish_and_record(&mut self) -> bool {
        match self.get_current_finish_status() {
            WonByPlayer::X => {
                self.won_by = WonByPlayer::X;
                return true;
            }
            WonByPlayer::O => {
                self.won_by = WonByPlayer::O;
                return true;
            }
            WonByPlayer::Tie => {
                self.won_by = WonByPlayer::Tie;
                return true;
            }
            WonByPlayer::HasntFinished => return false,
        }
    }



    fn is_finished(&self) -> bool{
        return match self.won_by{
            WonByPlayer::X => true,
            WonByPlayer::O => true,
            WonByPlayer::Tie => true,
            WonByPlayer::HasntFinished => false,
        }
    }
}

struct BigBoard {
    boards: [SmallBoard; 9],
    x_bits: u16,
    o_bits: u16,
    filled_boards: u8,
    last_sent_board_index: Option<u8>,
}

impl Display for BigBoard {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{}|{}\n\
             {}|{}|{}\n\
             {}|{}|{}",
            self.boards[0],
            self.boards[1],
            self.boards[2],
            self.boards[3],
            self.boards[4],
            self.boards[5],
            self.boards[6],
            self.boards[7],
            self.boards[8]
        )
    }
}

impl BigBoard {
    const fn new() -> Self {
        const INIT: SmallBoard = SmallBoard::new();
        Self { boards: [INIT; 9], last_sent_board_index: None, x_bits: 0, o_bits: 0, filled_boards: 0 }
    }
    fn current_move_can_be_put_anywhere(&self)->bool{
        self.last_sent_board_index.is_none() || self.boards[usize::from(self.last_sent_board_index.unwrap())].is_finished()
        
    }
    fn make_move(&mut self, small_board_pos: u8, tile_pos: u8, player: &Player) -> Result<WonByPlayer, BoardError> {
        if !self.current_move_can_be_put_anywhere() && small_board_pos!=self.last_sent_board_index.unwrap(){
            return Err(BoardError::MoveAtNotSentBoard) //illegal board
        }

        if self.current_move_can_be_put_anywhere() && self.boards[usize::from(small_board_pos)].is_finished(){
            return Err(BoardError::MoveAtAlreadyFinishedBoard)
        }

        //linter dies so type annotation needed
        let small_board_finished: &WonByPlayer=self.boards[usize::from(small_board_pos)].place_tile_and_record_finish(tile_pos, player)?; //return BoardError:MoveAtAlreadyFilledTile if errors out
        

        match small_board_finished{
            WonByPlayer::X => {self.x_bits ^= 1 << small_board_pos; self.filled_boards+=1},
            WonByPlayer::O => {self.o_bits ^= 1 << small_board_pos; self.filled_boards+=1},
            WonByPlayer::Tie =>self.filled_boards+=1,
            WonByPlayer::HasntFinished => (),
        }

        
        self.last_sent_board_index = Some(tile_pos);
        
        return Ok(self.get_current_finish_status())
    }

    fn check_draw(&self) -> bool {
        if self.filled_boards == 9 {
            return true;
        } else {
            return false;
        }
    }

    fn check_win(&self) -> WonByPlayer {
        // since these values are being accessed more than once it would
        // probably be a better idea to cache them or use a different structure
        // and then apply 2 bitmasks to check if either X or O has won.

        // on that now, should probably get git to work by now

        if self.x_bits & 0b001001001 == 0b001001001 // vertical
            || self.x_bits & 0b010010010 == 0b010010010
            || self.x_bits & 0b100100100 == 0b100100100
            || self.x_bits & 0b000000111 == 0b000000111 //horizontal
            || self.x_bits & 0b000111000 == 0b000111000
            || self.x_bits & 0b111000000 == 0b111000000
            || self.x_bits & 0b100010001 == 0b100010001 //diagonal
            || self.x_bits & 0b001010100 == 0b001010100
        {
            return WonByPlayer::X;
        } else if self.o_bits & 0b001001001 == 0b001001001 //vertical
            || self.o_bits & 0b010010010 == 0b010010010
            || self.o_bits & 0b100100100 == 0b100100100
            || self.o_bits & 0b000000111 == 0b000000111 //horizontal
            || self.o_bits & 0b000111000 == 0b000111000
            || self.o_bits & 0b111000000 == 0b111000000
            || self.o_bits & 0b100010001 == 0b100010001 //diagonal
            || self.o_bits & 0b001010100 == 0b001010100
        {
            return WonByPlayer::O;
        }
        return WonByPlayer::HasntFinished;
    }

    fn get_current_finish_status(&self) -> WonByPlayer {
        if self.check_draw() {
            return WonByPlayer::Tie;
        }
        return self.check_win();
    }

    const fn horizontal_seperator() -> &'static str{
        "═══════════╬═══════════╬═══════════"
    }
    #[rustfmt::skip]
    fn get_row(&self, row: u8) -> String {
        let index_usize= usize::from(row);
        return format!(
            "{}║{}║{}\n\
            {}\n\
             {}║{}║{}\n\
            {}\n\
             {}║{}║{}",
            self.boards[3*index_usize].get_row(0),
            self.boards[3*index_usize+1].get_row(0),
            self.boards[3*index_usize+2].get_row(0),
        
            SmallBoard::horizontal_seperator(),
            
            self.boards[3*index_usize].get_row(1),
            self.boards[3*index_usize+1].get_row(1),
            self.boards[3*index_usize+2].get_row(1),
            
            SmallBoard::horizontal_seperator(),
            
            self.boards[3*index_usize].get_row(2),
            self.boards[3*index_usize+1].get_row(2),
            self.boards[3*index_usize+2].get_row(2),
        );

    }
    #[rustfmt::skip]
    fn get_board(&self) -> String {
        return format!(
            "{}\n\
             {}\n\
             {}\n\
             {}\n\
             {}",

            self.get_row(0),
            BigBoard::horizontal_seperator(),
            self.get_row(1),
            BigBoard::horizontal_seperator(),
            self.get_row(2),
        
        );
    }
}

pub struct Game {
    game: BigBoard,
    next_player: Player,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.game.get_board())
    }
}

impl Game {
    pub const fn new() -> Self {
        Game {
            game: BigBoard::new(),
            next_player: Player::X,
        }
    }

    pub const fn get_next_player(&self) -> &Player{
        &self.next_player
    }

    pub fn print_game(&self){
        println!("{}", self);
    }

    fn switch_next_player(&mut self){
        self.next_player = match self.next_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    } 

    pub fn current_move_can_be_put_anywhere(&self) -> bool{
        self.game.current_move_can_be_put_anywhere()
    }

    pub fn last_sent_board_index(&self) -> Option<u8>{
        self.game.last_sent_board_index
    }

    pub fn make_move(&mut self, small_board_pos: u8, tile_pos: u8) -> Result<WonByPlayer, BoardError> {
        let successful=self.game.make_move(small_board_pos, tile_pos, &self.next_player);
        if successful.is_ok(){self.switch_next_player();}

        return successful;
    }
}