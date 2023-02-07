use std::io;


fn hello_players() {
    println!("-----------------------------------");
    println!("| Welcome to tic tac toe in Rust! |");
    println!("-----------------------------------");
    println!("");
}


fn print_board(board: Vec<Vec<String>>) {
    // function prints the current board state
    println!("   1   2   3");
    println!("");
    for i in 0..3 {
        println!("{}  {} | {} | {}", i+1, board[i][0], board[i][1], board[i][2]);
        if i < 2 {
            println!("   ---------");
        }
    }
    println!("")
}


fn get_move() -> Vec<i32> {
    // gets move from the user
    let mut input = String::new();
    let mut result: Vec<i32> = vec![0, 0];
    println!("enter move (row,column):");
    loop {
        io::stdin().read_line(&mut input).unwrap();
        let split = input.split(',').collect::<Vec<&str>>();
        if split.len() == 2 {
            let x = split[0].trim().parse::<i32>();
            let y = split[1].trim().parse::<i32>();
            if x.is_ok() && y.is_ok() {
                result[0] = x.unwrap();
                result[1] = y.unwrap();
                break;
            }
        }
        println!("Invalid move, try again.");
        input.clear();
    }
    return result;
}

fn check_horizontal(board: &Vec<Vec<String>>) -> bool {
    // checks if one row have 3 of the same character
    for i in 0..3 {
        // check if all the same
        if (board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != " ") {
            return true;
        }
    }
    return false;
}

fn check_vertical(board: &Vec<Vec<String>>) -> bool {
    // checks if any column has all three of the same characters
    for i in 0..3 {
        // check if all the same
        if (board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != String::from(" ")) {
            return true;
        }
    }
    return false;
}

fn check_diagaonal(board: &Vec<Vec<String>>) -> bool {
    // check the diagaonal for all matching characters
    if (board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != String::from(" ")) {
        return true;
    }
    if (board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != String::from(" ")) {
        return true;
    }
    return false;
}

fn game_over(board: &Vec<Vec<String>>) -> bool {
    // figure out if the game is over
    if check_diagaonal(board) || check_vertical(board) || check_horizontal(board) {
        return true;
    }
    return false;
}


fn main() {
    // say hello to our players
    hello_players();
    // define the board and init with empty state
    let mut board: Vec<Vec<String>> = vec![vec![String::from(" "); 3]; 3];
    let mut m: Vec<i32> = vec![0, 0];
    // print the board
    print_board(board.clone());
    // turn is x
    let mut turn = "X";
    // loop for every possible round
    loop {
        println!("");
        print_board(board.clone());
        println!("");

        m = get_move();
        board[m[1] as usize - 1][m[0] as usize - 1] = turn.to_string();

        if game_over(&board) {
            print_board(board.clone());
            println!("Game over! {} wins!", turn);
            break;
        }
        turn = if turn == "O" {"X"} else {"O"};
        println!("It's now {}'s turn", turn);
    }
}




