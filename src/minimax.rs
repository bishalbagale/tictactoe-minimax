use crate::game_status;

pub fn next_move(board: &mut Vec<char>, player: char) -> i32 {
    let mut best_score = if player == 'X' { i32::MIN } else { i32::MAX };
    let mut best_move: i32 = -1;
    let mut score: i32;
    for possible_move in possible_moves(board) {
        board[possible_move] = player;
        score = minimax(board, 0, player == 'O');
        board[possible_move] = ' ';

        if player == 'X' {
            if score > best_score {
                best_score = score;
                best_move = possible_move as i32;
            }
        } else {
            if score < best_score {
                best_score = score;
                best_move = possible_move as i32;
            }
        }
    }

    return best_move;
}

fn possible_moves(board_vector: &Vec<char>) -> Vec<usize> {
    let mut possible_moves = Vec::new();
    for i in 0..9 {
        if board_vector[i] == ' ' {
            possible_moves.push(i);
        }
    }
    possible_moves
}

fn minimax(board: &mut Vec<char>, depth: i32, is_maximizing: bool) -> i32 {
    //let mut board = board_vector.clone();
    let winner = game_status::status(board);
    if winner == 'X' {
        return 10 - depth;
    } else if winner == 'O' {
        return depth - 10;
    } else if winner == 'T' {
        return 0;
    };

    if is_maximizing {
        let mut best_score = i32::MIN;
        for i in possible_moves(board) {
            board[i] = 'X';
            let score = minimax(board, depth + 1, false);
            board[i] = ' ';
            best_score = best_score.max(score); //max(best_score, score)
            return best_score;
        }
    } else {
        let mut best_score = i32::MAX;
        for i in possible_moves(board) {
            board[i] = 'O';
            let score = minimax(board, depth + 1, true);
            board[i] = ' ';
            best_score = best_score.min(score); //min(best_score, score)
            return best_score;
        }
    }

    0
}

