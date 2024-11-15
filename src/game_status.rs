//use crate::Player;
//
//pub fn win_checker(player: &Player) -> char {
//    let win_conditions: [[usize; 3]; 8] = [
//        // Rows
//        [0, 1, 2],
//        [3, 4, 5],
//        [6, 7, 8],
//        // Columns
//        [0, 3, 6],
//        [1, 4, 7],
//        [2, 5, 8],
//        // Diagonals
//        [0, 4, 8],
//        [2, 4, 6],
//    ];
//
//    let mut winner = ' ';
//    //check win conditions
//    for condition in win_conditions.iter() {
//        //println!("{:?}", condition);
//        if condition.iter().all(|&cell| player.cells.contains(&cell)) {
//            winner = player.symbol;
//        }
//    }
//    winner
//}
//
//pub fn tie_checker(board_vector: &Vec<char>) -> bool {
//    //check for tie
//    let mut tie_status = false;
//    for i in 0..9 {
//        if board_vector[i] == ' ' {
//            tie_status = false;
//            break;
//        } else {
//            tie_status = true;
//        }
//    }
//    tie_status
//}
//
//
//
//
//

pub fn status(board: &Vec<char>) -> char {
    //check row
    let mut status: char = ' ';
    for i in (0..9).step_by(3) {
        if board[i] != ' ' {
            if board[i] == board[i + 1] && board[i + 1] == board[i + 2] {
                status = board[i];

                return status;
            }
        };
    }

    //column check
    for i in 0..3 {
        // println!("value of i: {}", j);
        if board[i] != ' ' {
            if board[i] == board[i + 3] && board[i + 3] == board[i + 6] {
                status = board[i];

                return status;
            }
        }
    }

    //check  diag
    if board[0] == board[4] && board[4] == board[8] {
        status = board[0];

        return status;
    } else if board[2] == board[4] && board[4] == board[6] {
        status = board[2];
        return status;
    }

    //tie
    for i in 0..9 {
        if board[i] == ' ' {
            status = ' ';
            break;
        } else {
            status = 'T';
        }
    }

    status
    // winner
}
