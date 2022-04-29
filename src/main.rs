use std::{
    fmt,
    io::{stdin, stdout, Write},
};

fn print_game_board(board: &[&str; 9]) {
    (0..9).for_each(|i| {
        print!("[{}]", board[i]);
        if i == 3 || i == 6 {
            println!();
        }
    });
    println!();
    stdout().flush().unwrap();
}

fn read_input(board: &mut [&'static str; 9], symbol: &'static str) -> Result<(), ()> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Bad input!");
    let ch = input.chars().next().ok_or(())?;
    if ch.is_digit(10) && board[map_input(ch)] == " " {
        let BoardIndex(index) = ch.into();

        board[BoardIndex::from(ch).to_inner()] = symbol;
        Ok(())
    } else {
        println!("Bad input, the field is not empty !");
        Err(())
    }
}

struct BoardIndex(usize);
impl From<char> for BoardIndex {
    fn from(input: char) -> Self {
        let num = match input {
            '7' => 0,
            '8' => 1,
            '9' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '1' => 6,
            '2' => 7,
            '3' => 8,
            _ => 10,
        };
        Self(num)
    }
}

impl BoardIndex {
    fn to_inner(self) -> usize {
        let BoardIndex(result) = self;
        result
    }
}

fn map_input(input: char) -> usize {
    match input {
        '7' => 0,
        '8' => 1,
        '9' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '1' => 6,
        '2' => 7,
        '3' => 8,
        _ => 10,
    }
}

fn check_win_conditions(board: &[&str; 9]) -> bool {
    let mut victory: bool = false;

    let row_elements = [0, 3, 6];

    for element in row_elements {
        if board[element] != " "
            && board[element] == board[element + 1]
            && board[element] == board[element + 2]
        {
            victory = true;
        }
    }

    let col_elements = [0, 1, 2];
    for element in col_elements {
        if board[element] != " "
            && board[element] == board[element + 3]
            && board[element] == board[element + 6]
        {
            victory = true;
        }
    }

    let diag_element = 4;
    if board[diag_element] != " "
        && board[diag_element] == board[diag_element - 4]
        && board[diag_element] == board[diag_element + 4]
    {
        victory = true;
    }

    if board[diag_element] != " "
        && board[diag_element] == board[diag_element - 2]
        && board[diag_element] == board[diag_element + 2]
    {
        victory = true;
    }

    victory
}

// fn is_draw(board: &[&str; 9]) -> bool {
//     !board.iter().any(|ele| ele == &" ")
// }

fn is_draw(board: &[&str; 9]) -> bool {
    let mut is_draw = true;
    for ele in board {
        if ele == &" " {
            is_draw = false;
            break;
        }
    }
    is_draw
}

fn main() {
    let mut game_board: [&'static str; 9] = [" ", " ", " ", " ", " ", " ", " ", " ", " "];
    let mut win_status: bool = false;

    println!("Hello, gamers!\nUse the numpad to place X / O\nand play tic tac toes !");

    let mut player_symbol: &str;
    let mut is_player_x = true;
    while !win_status {
        if is_player_x {
            player_symbol = "X";
        } else {
            player_symbol = "O";
        }

        println!("{} Plays next !", player_symbol);

        print_game_board(&game_board);

        if read_input(&mut game_board, player_symbol).is_ok() {
            is_player_x = !is_player_x;
        }

        win_status = check_win_conditions(&game_board);

        if is_draw(&game_board) {
            break;
        }
    }

    match win_status {
        true => println!("WINNER!"),
        false => println!("DRAW!"),
    }

    print_game_board(&game_board);
}
