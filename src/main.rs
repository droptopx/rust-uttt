use std::io::{self, Write};
use uttt_game::BoardError;

mod uttt_game;

/* This code needs to be refactored hard, maybe move main loop into another file or perhaps in uttt_game.rs?
 * Actually, I might need to redo all of this project because its way too mangled up
 * 
 * */
fn main() {
    let mut game = uttt_game::Game::new();
    //game.make_move(0, 1, uttt_game::Player::X);
    game.print_game();
    loop {
        let mut input = String::new();
        let big_board_index = if game.current_move_can_be_put_anywhere() {
            println!(
                "[{}] You can put your tile on any board",
                game.get_next_player().get_letter()
            );
            print!(
                "[{}] Big board index: ",
                game.get_next_player().get_letter()
            );
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("[!!!] Unexpected crash happened");
            let input_trimmed = input.trim();

            if input_trimmed.len() != 1 {
                println!("[!] Input one character");
                continue;
            }

            match input_trimmed.parse::<u8>() {
                Ok(val) => val,
                Err(_) => {
                    println!("[!] Input a number");
                    continue;
                }
            }
        } else {
            println!(
                "[{}] You have to put your tile on board #{}",
                game.get_next_player().get_letter(),
                game.last_sent_board_index().unwrap()
            ); //shouldnt panic as the None variant is ruled out in !game.current_move_can_be_put_anywhere()
            game.last_sent_board_index().unwrap()
        };

        print!(
            "[{}] Small board index: ",
            game.get_next_player().get_letter()
        );
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("[!!!] Unexpected crash happened");
        let input_trimmed = input.trim();

        if input_trimmed.len() != 1 {
            println!("[!] Input one character");
            continue;
        }

        let small_board_index = match input_trimmed.parse::<u8>() {
            Ok(val) => val,
            Err(_) => {
                println!("[!] Input a number");
                continue;
            }
        };

        let game_status = game.make_move(big_board_index, small_board_index);
        if let Err(error) = game_status {
            match error{
                BoardError::MoveAtAlreadyFilledTile => println!("[!] That tile was already taken"),
                BoardError::MoveAtNotSentBoard => println!("[!] You can not make a move at that board as you weren't sent there"),
                BoardError::MoveAtAlreadyFinishedBoard => println!("[!] You can not make a move at that board as it has been completed"),
            }
            continue;
        }

        
        print!("\x1B[2J\x1B[1;1H"); //clear screen
        game.print_game();
        match game_status.unwrap() {
            uttt_game::WonByPlayer::X => {
                println!("[#] Game won by X");
                break;
            }
            uttt_game::WonByPlayer::O => {
                println!("[#] Game won by O");
                break;
            }
            uttt_game::WonByPlayer::Tie => {
                println!("[#] Game tied");
                break;
            }
            uttt_game::WonByPlayer::HasntFinished => (),
        };
    }
}
