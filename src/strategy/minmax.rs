//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use rand::Rng;


/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        fn minimax(state: &Configuration, profondeur : u8) -> (i8, Option<Movement>) {
            if state.movements().next().is_some() && profondeur > 0 {
                
                    let mut score_opt = i8::MAX;
                    let mut list_move_opt: Vec<Movement> = Vec::new();

                    for mouvement in state.movements() {
                        //let mut etat = state.play(&mouvement);
                        //score_opt = cmp::max(score_opt, minimax(&etat, profondeur-1, false));
                        let res = minimax(&state.play(&mouvement), profondeur-1);
                        //print!("MinMax :scoreOpt={}, scoreNouv={}\n", score_opt, res.0);
                        if score_opt > -res.0 {
                            //print!("MinMax :scoreOpt={}, scoreNouv={}\n", score_opt, -res.0);
                            score_opt = -res.0; 
                            list_move_opt.push(mouvement);
                        } 
                    }
                    let randIndex = rand::thread_rng().gen_range(0..list_move_opt.len());
                    (score_opt, Some(list_move_opt[randIndex]))

                
            }else {
                (state.value(), None) //return value ce node
            }
        }
        
        minimax(state, self.0).1
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
