extern crate rand;

use std::env;
use std::str::FromStr;
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DoorChoice {
    Left,
    Center,
    Right,
}

use DoorChoice::*;

impl rand::Rand for DoorChoice {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        *rng.choose(&[Left, Center, Right]).unwrap()
    }
}

mod game {
    use super::DoorChoice;
    use DoorChoice::*;
    use rand::{Rng, self};

    pub struct GameMaster {
        goal: DoorChoice,
    }

    pub struct GameRevealed {
        goal: DoorChoice,
    }

    impl GameMaster {
        pub fn new() -> Self {
            let mut rng = rand::thread_rng();
            GameMaster {
                goal: rng.gen(),
            }
        }

        pub fn reveal_empty(self, user_choice: DoorChoice) -> (GameRevealed, DoorChoice) {
            for &choice in &[Left, Center, Right] {
                if choice != user_choice && choice != self.goal {
                    return (GameRevealed { goal: self.goal }, choice);
                }
            }
            unreachable!()
        }
    }

    impl GameRevealed {
        pub fn is_victorious(self, user_choice: DoorChoice) -> bool {
            user_choice == self.goal
        }
    }
}

fn naive_player_wins() -> bool {
    let oracle = game::GameMaster::new();
    let mut rng = rand::thread_rng();
    let choice = rng.gen();
    let (oracle, _revealed) = oracle.reveal_empty(choice);
    oracle.is_victorious(choice)
}

fn smart_player_wins() -> bool {
    let oracle = game::GameMaster::new();
    let mut rng = rand::thread_rng();
    let mut choice = rng.gen();
    let (oracle, revealed) = oracle.reveal_empty(choice);
    debug_assert!(choice != revealed);
    for &door in &[Left, Center, Right] {
        if door != choice && door != revealed {
            choice = door;
            break;
        }
    }
    oracle.is_victorious(choice)
}

fn main() {
    let nb_tries = if let Some(nb_tries_arg) = env::args().skip(1).next() {
        FromStr::from_str(&nb_tries_arg).expect("nb tries should be an integer")
    } else {
        1000
    };

    let mut nb_wins_smart = 0;
    let mut nb_wins_naive = 0;
    for _ in 0..nb_tries {
        if smart_player_wins() {
            nb_wins_smart += 1;
        }
        if naive_player_wins() {
            nb_wins_naive += 1;
        }
    }
    println!("nb smart player wins: {} / {}", nb_wins_smart, nb_tries);
    println!("nb naive player wins: {} / {}", nb_wins_naive, nb_tries);
}
