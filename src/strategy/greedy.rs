//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use std::fmt;
use rand::Rng;

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        if state.movements().next().is_some(){ 
            let mut score_opt = i8::MIN;
            let mut list_move_opt: Vec<Movement> = Vec::new();

            for mouvement in state.movements() {
                let mut score = state.play(&mouvement).value(); 
                if score_opt < score {
                    print!("Greedy : scoreAct={}, scoreNouv={}\n", score_opt, score);
                    score_opt = score; 
                    list_move_opt.push(mouvement);
                } 
            }
            let randIndex = rand::thread_rng().gen_range(0..list_move_opt.len());
            Some(list_move_opt[randIndex])
        
            
        }else{
            None
        }
    }
}
