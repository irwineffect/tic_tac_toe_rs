use std::io;
/*
 * 1 | 2 | 3
 *-----------
 * 4 | 5 | 6
 *-----------
 * 7 | 8 | 9
 *
 */

fn draw_board(board: &[char]) {
    let horizontal_line = "-----------";
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("{}", horizontal_line);
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("{}", horizontal_line);
    println!(" {} | {} | {}", board[6], board[7], board[8]);
}

fn valid_move(board: &[char], player_move: i8) -> bool {
    if 0 <= player_move && player_move < 9 {
        if ((board[player_move as usize] as u8) - ('0' as u8)) < 10 {
            true
        }
        else
        {
            false
        }
    } else {
        false
    }

}

#[derive(PartialEq)]
enum GAME_STATE {
    IN_PROGRESS,
    PLAYER_1_WINS,
    PLAYER_2_WINS,
    STALE_MATE
}

// TODO: need to properly check for game over condition
fn game_over(board: &[char]) -> GAME_STATE {
    GAME_STATE::IN_PROGRESS
}

fn main() {
    let mut board: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player_selector: bool = true;
    let mut game_state = GAME_STATE::IN_PROGRESS;


    while game_state == GAME_STATE::IN_PROGRESS {
        loop {
            draw_board(&board);
            println!("player {} select move > ", player_selector);
            let mut player_move = String::new();
            io::stdin().read_line(&mut player_move)
                        .expect("Failed to read line");
            let player_move: i8 = match player_move.trim().parse() {
                Ok(t) => t,
                Err(e) => {println!("{}", e); -1}
            };
            let player_move = player_move - 1;
            if ! valid_move(&board, player_move) {
                println!("invalid move");
            }
            else {
                board[player_move as usize] = if player_selector { 'X' } else {'Y'};
                break;
            }
        }
        game_state =  game_over(&board);
        player_selector = !player_selector;
    };
}
