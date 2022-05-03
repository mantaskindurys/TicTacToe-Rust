use std::{
    fs::{read_to_string, File},
    io::{stdin, stdout, Error, Write},
    str,
};

const PATH: &str = "game_save.txt";

fn save_game_state(board: &[char; 9], player: &char) {
    let mut file = File::create(PATH).expect("Failed opening or creating file.");
    let data: String = board.iter().collect();
    write!(file, "{}{}", data, player).expect("failed to write to file.");
    
}

fn load_game_state(board: &mut [char; 9]) -> Result<bool, Error> {
    let data = {
        let this = read_to_string(PATH);
        match this {
            Ok(t) => t,
            Err(e) => return Err(e),
        }
    };

    let result = data.chars();
    let mut index = 0;
    let mut player_symbol = false;
    for i in result {
        if index < 9 {
            board[index] = i;
            println!("{}", i);
        } else {
            player_symbol = i=='X';
            break;
        }
        index += 1;
    }

    Ok(player_symbol)
}

fn print_game_board(board: &[char; 9]) {
    (0..9).for_each(|i| {
        print!("[{}]", board[i]);
        if i == 2 || i == 5 {
            println!();
        }
    });
    println!();
    stdout().flush().unwrap();
}

fn read_input(board: &mut [char; 9], symbol: char) -> Result<char, ()> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Bad input!");
    let ch = input.chars().next().ok_or(())?;
    if ch == 'X'{
        save_game_state(board, &symbol);
        return Ok(ch);
    }
    if ch.is_digit(10) && board[map_input(ch)] == ' ' {
        let BoardIndex(index) = ch.into();

        board[BoardIndex::from(ch).to_inner()] = symbol;
        Ok(ch)
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

fn check_win_conditions(board: &[char; 9]) -> bool {
    let mut victory: bool = false;

    let row_elements = [0, 3, 6];

    for element in row_elements {
        if board[element] != ' '
            && board[element] == board[element + 1]
            && board[element] == board[element + 2]
        {
            victory = true;
        }
    }

    let col_elements = [0, 1, 2];
    for element in col_elements {
        if board[element] != ' '
            && board[element] == board[element + 3]
            && board[element] == board[element + 6]
        {
            victory = true;
        }
    }

    let diag_element = 4;
    if board[diag_element] != ' '
        && board[diag_element] == board[diag_element - 4]
        && board[diag_element] == board[diag_element + 4]
    {
        victory = true;
    }

    if board[diag_element] != ' '
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

fn is_draw(board: &[char; 9]) -> bool {
    let mut is_draw = true;
    for ele in board {
        if ele == &' ' {
            is_draw = false;
            break;
        }
    }
    is_draw
}

enum GameStatus{
    Winner,
    Draw,
    Saved
}

fn main() {
    let mut game_board: [char; 9] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut win_status: bool = false;
    let mut status:GameStatus = GameStatus::Winner;

    println!("Hello, gamers!\nUse the numpad to place X / O\nand play tic tac toes !");

    let mut player_symbol:char;
    let mut is_player_x = true;

    println!("Try loading saved game Y/N ?");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed reading user input.");
    
    if buf.trim()=="Y"||buf.trim()=="y"{
        match load_game_state(&mut game_board) {
            Ok(symb) => is_player_x = symb,
            Err(_e) => println!("Loading saved game failed. Starting new game!"),
        }
    }

    while !win_status {
        if is_player_x {
            player_symbol = 'X';
        } else {
            player_symbol = 'O';
        }

        println!("{} Plays next ! Press X to save and exit.", player_symbol);

        print_game_board(&game_board);
        let res = read_input(&mut game_board, player_symbol);
        if let Ok(res) = res {
            if res == 'X'{
                status=GameStatus::Saved;
                break;
            }
            is_player_x = !is_player_x;
        }

        win_status = check_win_conditions(&game_board);

        if is_draw(&game_board) {
            status=GameStatus::Draw;
            break;
        }
    }

    match status {
        GameStatus::Winner => println!("WINNER!"),
        GameStatus::Draw => println!("DRAW."),
        GameStatus::Saved => println!("Game saved and closing.")
    }

    print_game_board(&game_board);
}
