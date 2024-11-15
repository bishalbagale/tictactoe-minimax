use clearscreen::clear;
//use rand::rngs::adapter;
use rand::Rng;
use std::io;

use term_table::row::Row;
use term_table::table_cell::TableCell;

mod game_status;
mod minimax;
struct Player {
    symbol: char,
    cells: Vec<usize>, //status: String,
}

fn main() {
    //initialize players
    let mut player1 = Player {
        symbol: 'X',
        cells: Vec::with_capacity(5), // 5 is the max no of moves a player1 is able to make.
                                      //status: String::from(""),
    };
    let mut player2 = Player {
        symbol: 'O',
        cells: Vec::with_capacity(4), // 4 is the max no of moves a player2 is able to make.
                                      //status: String::from(""),
    };
    // init board vector with empty cells
    let mut board_vector: Vec<char> = vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    draw_board_table(&mut board_vector);

    game_logic(&mut board_vector, &mut player1, &mut player2);
}

fn game_logic(board_vector: &mut Vec<char>, player1: &mut Player, player2: &mut Player) {
    loop {
        //take user input
        println!("Player Input [1-9]");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read stdin");
        //parse to integer
        let parsed_user_input: usize = user_input.trim().parse().expect("Invalid Input.");
        //check if user input is in range of available cells i.e. 0-9
        if parsed_user_input > 9 {
            println!("Input out of range.");
            continue;
        } else {
            match board_vector[parsed_user_input - 1] {
                ' ' => {
                    board_vector[parsed_user_input - 1] = player1.symbol;
                    player1.cells.push(parsed_user_input - 1);
                }

                _ => {
                    println!("Cell Already Occupied.");
                    continue;
                }
            }
        }
        let game_status = game_status::status(board_vector);

        match game_status {
            'X' => {
                draw_board_table(board_vector);
                println!("Player1 won");
                break;
            }

            'T' => {
                draw_board_table(board_vector);
                println!("Tie");
                break;
            }
            _ => {
                draw_board_table(board_vector);
            }
        }

        player2_move(board_vector, player2);

        let game_status = game_status::status(board_vector);

        match game_status {
            'O' => {
                draw_board_table(board_vector);
                println!("Player2 won");
                break;
            }

            'T' => {
                draw_board_table(board_vector);
                println!("Tie");
                break;
            }
            _ => {
                draw_board_table(board_vector);
            }
        }
    }
}

fn player2_move(board_vector: &mut Vec<char>, player2: &mut Player) {
    let minmax_move = minimax::next_move(board_vector, player2.symbol);
    println!("computer move -> {:?}", minmax_move);
    let mut rng = rand::thread_rng();
    let mut random_move;
    //board_vector[minmax_move as usize] = player2.symbol;
    //player2.cells.push(minmax_move as usize);
    loop {
        random_move = rng.gen_range(0..9);
        if board_vector[random_move] == ' ' {
            board_vector[minmax_move as usize] = player2.symbol;
            player2.cells.push(minmax_move as usize);
            break;
        }
    }
}

fn draw_board_table(board_vector: &mut Vec<char>) {
    clear().unwrap();
    let mut table = term_table::Table::new();
    // table.max_column_width = 20;
    table.style = term_table::TableStyle::extended();
    table.add_row(Row::new(vec![
        TableCell::new(board_vector[0]),
        TableCell::new(board_vector[1]),
        TableCell::new(board_vector[2]),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new(board_vector[3]),
        TableCell::new(board_vector[4]),
        TableCell::new(board_vector[5]),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new(board_vector[6]),
        TableCell::new(board_vector[7]),
        TableCell::new(board_vector[8]),
    ]));
    println!("{}", table.render());
}
