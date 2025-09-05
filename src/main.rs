use std::{io, cmp::Ordering, ops::Range};
use rand::Rng;

const OBSTACLE: &str = "┗| `O′|┛";
const SAFE_SPACE: &str = "_";
const BOARD_LEN: usize = 32;

fn main() {
    
    let board = make_board(BOARD_LEN);

    // Player init board placement is outside
    let mut current_place: usize = 0;
    let mut first_roll = true;

    // Display board without the player current place
    print_board(&board.len(), &board);

    'pitfall: loop {
        
        // 1. Roll dice
        let rolls: Rolls = roll(1..7, 1..7);
        let roll: usize = [rolls.0, rolls.1].iter().sum();
        let turn: PlayerTurn = PlayerTurn { current_place, roll };

        println!("\t>>> {:?}", turn);

        // 2. Find next place to move in the board
        let next_place = match find_next_place(&turn, first_roll) {
            NextPlace::Place(p) => p,
            NextPlace::Goal => break 'pitfall
        };

        // 3. Move player
        current_place = move_player(&board, next_place);
        
        // 4. Print board
        print_board(&current_place, &board);

        first_roll = false;
        println!("======\n");
    }
}


// 
/*  */

type Board = Vec<Place>;
type Rolls = (usize, usize);

#[derive(Clone, Copy, Debug)]
enum Place {
    Obstacle(usize), // obstacle with # steps back (penalty)
    Safe
}

#[derive(Debug)]
enum NextPlace {
    Place(usize), // Place index
    Goal
}

#[derive(Debug)]
struct PlayerTurn {
    current_place: usize,
    roll: usize
}


// 
/*  */
fn make_board(len: usize) -> Board {
    
    let mut i = 0;
    let mut board : Board = vec![];

    while i < len {

        // 0: Safe, 1: Obstacle
        let place: i8 = rand::random_range(0..=1);

        if place == 0 { board.push(Place::Safe); }
        else {
            // steps back
            let penalty: usize = rand::random_range(2..4);

            board.push(Place::Obstacle(penalty));
        }

        i+=1;
    }

    board
}

/*  */
fn print_board(player_place: &usize, board: &Board) -> () {
    
    let mut board_display: Vec<String> = vec![];

    for (i, &place) in board.iter().enumerate() {

        let display = match place {
            Place::Obstacle(_) => OBSTACLE.to_string(),
            Place::Safe => SAFE_SPACE.to_string()
        };

        if i == *player_place { board_display.push(format!(">{}<", i+1)); }
        else                  { board_display.push(display); }
    }

    println!("{:?}", board_display)
}

/*  */
fn roll(dice_1: Range<usize>, dice_2: Range<usize>) -> Rolls {
    
    println!("{{#}}{{#}} Roll two dice…");

    // Just for waiting, not actually using input
    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");

    (rand::random_range(dice_1), rand::random_range(dice_2))
}

/*  */
fn find_next_place(turn: &PlayerTurn, first_roll: bool) -> NextPlace {
    
    let next_place = turn.current_place + turn.roll;

    if next_place >= BOARD_LEN  { NextPlace::Goal }
    else if first_roll          { NextPlace::Place(turn.roll - 1) }
    else                        { NextPlace::Place(next_place) }
}

/*  */
fn move_player(board: &Board, next_place: usize) -> usize {
    
    let place = board.get(next_place);

    match place {
        Some(Place::Obstacle(penalty)) => {

            println!("A monster appeared at {}, you move {} steps back to survive:", next_place + 1, penalty);

            match next_place.cmp(penalty) {
                Ordering::Less => 0, // Don't go outside board
                _ => next_place - *penalty
            }
        },
        Some(Place::Safe) => next_place,
        None => next_place,
    }
}
