use std::{io, cmp::Ordering, ops::Range};
use rand;
use std::fmt;


const BOARD_LEN: usize = 32;

fn main() {
    
    let mut board = make_board(BOARD_LEN);

    // Player init board placement is outside
    let mut current_place: usize = 0;
    let mut first_roll = true;

    // Display board without the player current place
    println!("{}", board);

    'pitfall: loop {
        
        // 1. Roll dice
        let rolls: Rolls = roll(1..7, 1..7);
        let roll: usize = [rolls.0, rolls.1].iter().sum();
        let turn: PlayerTurn = PlayerTurn { current_place, roll };

        println!("{}", turn);

        // 2. Find next place to move in the board
        let find_next_place = find_next_place(&turn, first_roll);
        println!("Next place position:{}", find_next_place);

        let next_place = match find_next_place {
            NextPlace::Place(p) => p,
            NextPlace::Goal => break 'pitfall
        };    
        
        // 3. Move player
        current_place = move_player(&mut board, next_place);
        
        // 4. Print board
        println!("{}", board);

        first_roll = false;
        println!("======\n");
    }

    println!("You Won!\n");
}


// 
/*  */

type Rolls = (usize, usize);

#[derive(Clone, Copy)]
enum Place {
    Obstacle(usize), // obstacle with # steps back (penalty)
    Safe
}
impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            Self::Obstacle(penatly) => writeln!(f, "   ┗| `O′|┛{}", penatly),
            Self::Safe => writeln!(f, "    |[  ]|")
        }
    }
}

enum NextPlace {
    Place(usize), // Place index
    Goal
}
impl fmt::Display for NextPlace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Place(position) => writeln!(f, "   |[ {} ]|", position),
            Self::Goal => writeln!(f, "   |[<*>]|")
        }
    }
}

struct Board{
    board: Vec<Place>,
    player_pos: usize
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for (i, &place) in self.board.iter().enumerate() {

            if i == self.player_pos { writeln!(f, "P-|[ >{}< ]|", i)?; }
            else                    { write!(f, "{}", place)?; }
        }

        writeln!(f, "---")
    }
}

struct PlayerTurn {
    current_place: usize,
    roll: usize
}
impl fmt::Display for PlayerTurn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ">>> Player Turn\n    >{}< | {{#{}}}", self.current_place, self.roll)
    }
}



// 
/*  */
fn make_board(len: usize) -> Board {
    
    let mut i = 0;
    let mut board = vec![];

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

    Board { board, player_pos: 0 }
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
fn move_player(board: &mut Board, next_place: usize) -> usize {
    
    let some_place = board.board.get(next_place);

    match some_place {
        Some(p@Place::Obstacle(penalty)) => {

            println!("A monster appeared at {}, you move {} steps back to survive:", next_place, penalty);
            println!("{}", p);

            match next_place.cmp(penalty) {
                Ordering::Less => 0, // Don't go outside board
                _ => {
                    // Update player board position
                    let penatly_place = next_place - *penalty;
                    board.player_pos = penatly_place;

                    penatly_place
                }
            }
        },
        Some(p@Place::Safe) => {
            println!("Next Place is safe!:{}", p);
            board.player_pos = next_place;
            next_place
        },
        None => {
            println!("None place");
            board.player_pos = next_place;
            next_place
        }
    }
}
