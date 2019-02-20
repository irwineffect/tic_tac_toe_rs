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
#[derive(Debug)]
enum GAME_STATE {
    IN_PROGRESS,
    PLAYER_1_WINS,
    PLAYER_2_WINS,
    STALE_MATE
}

// TODO: need to properly check for game over condition
fn game_over(board: &[char]) -> GAME_STATE {
    // Check if player 1 has won
    if board[0] == 'X' && board[1] == 'X' && board[2] == 'X' {
        return GAME_STATE::PLAYER_1_WINS;
    } else if board[3] == 'X' && board[4] == 'X' && board[5] == 'X' {
        return GAME_STATE::PLAYER_1_WINS;
    } else if board[6] == 'X' && board[7] == 'X' && board[8] == 'X' {
        return GAME_STATE::PLAYER_1_WINS;
    } else if board[0] == 'X' && board[3] == 'X' && board[6] == 'X' {
        return GAME_STATE::PLAYER_1_WINS;
    } else if board[1] == 'X' && board[4] == 'X' && board[7] == 'X' {
        return GAME_STATE::PLAYER_1_WINS;
    } else if board[2] == 'X' && board[5] == 'X' && board[8] == 'X' {
        return GAME_STATE::PLAYER_1_WINS;
    } else if board[0] == 'X' && board[4] == 'X' && board[8] == 'X' {
        return GAME_STATE::PLAYER_1_WINS;
    } else if board[2] == 'X' && board[4] == 'X' && board[6] == 'X' {
        return GAME_STATE::PLAYER_1_WINS;
    }
    // Check if player 2 has won
    else if board[0] == 'Y' && board[1] == 'Y' && board[2] == 'Y' {
        return GAME_STATE::PLAYER_2_WINS;
    } else if board[3] == 'Y' && board[4] == 'Y' && board[5] == 'Y' {
        return GAME_STATE::PLAYER_2_WINS;
    } else if board[6] == 'Y' && board[7] == 'Y' && board[8] == 'Y' {
        return GAME_STATE::PLAYER_2_WINS;
    } else if board[0] == 'Y' && board[3] == 'Y' && board[6] == 'Y' {
        return GAME_STATE::PLAYER_2_WINS;
    } else if board[1] == 'Y' && board[4] == 'Y' && board[7] == 'Y' {
        return GAME_STATE::PLAYER_2_WINS;
    } else if board[2] == 'Y' && board[5] == 'Y' && board[8] == 'Y' {
        return GAME_STATE::PLAYER_2_WINS;
    } else if board[0] == 'Y' && board[4] == 'Y' && board[8] == 'Y' {
        return GAME_STATE::PLAYER_2_WINS;
    } else if board[2] == 'Y' && board[4] == 'Y' && board[6] == 'Y' {
        return GAME_STATE::PLAYER_2_WINS;
    } else {
        // Check if the game is not a STALE_MATE
        for i in 0 .. 9 {
            if ((board[i as usize] as u8) - ('0' as u8)) < 10 {
                return GAME_STATE::IN_PROGRESS;
            }
        }
    }
    return GAME_STATE::STALE_MATE;
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
    draw_board(&board);
    println!("{:?}", game_state);
}
