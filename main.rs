use std::mem;

fn hello_players() {
    println!("-----------------------------------");
    println!("| Welcome to tic tac toe in Rust! |");
    println!("-----------------------------------");
    println!("");
}


fn print_board(board: Vec<Vec<String>>) {
    // function prints the current board state
    println!("   A   B   C");
    println!("");
    for i in 0..3 {
        println!("{}  {} | {} | {}", i+1, board[i][0], board[i][1], board[i][2]);
        if (i < 2) {
            println!("   ---------");
        }
    }
    println!("")
}


fn main() {
    
    // say hello to our players
    hello_players();

    // define the board and init with empty state
    let mut board: Vec<Vec<String>> = vec![vec![String::from(" "); 3]; 3];
   
    print_board(board);

}




