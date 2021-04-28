//! Alpha - Beta algorithm.
use std::fmt;

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::cmp;
use rand::Rng;
use std::collections::HashMap;


/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    let mut alpha = -127;
    let mut beta = 127;
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth, alpha, beta).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8, pub i8, pub i8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    /*fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        
        fn alpha_beta(state: &Configuration, profondeur : u8, mut alpha: i8, mut beta: i8) -> (i8, Option<Movement>){
            if state.movements().next().is_some() && profondeur > 0 {
                    
                    let mut score_opt = i8::MAX;
                    let mut list_move_opt: Vec<Movement> = Vec::new();
                    let mut val_list_opt = i8::MAX;
    
                    for mouvement in state.movements() {
                        //let mut etat = state.play(&mouvement);
                        //score_opt = cmp::max(score_opt, minimax(&etat, profondeur-1, false));
                        let res = -alpha_beta(&state.play(&mouvement), profondeur-1, -beta, -alpha).0; //pour optimiser ou minimiser selon si on joue ou l'adv
                        //print!("AlphaBeta :scoreOpt={}, scoreNouv={}\n", score_opt, res.0);
                        if score_opt > res {
                            //print!("AlphaBeta :scoreOpt={}, scoreNouv={}\n", score_opt, -res.0);
                            score_opt = res;
                            if score_opt == val_list_opt {
                                list_move_opt.push(mouvement);
                            } else{
                                val_list_opt = score_opt;
                                list_move_opt = Vec::new();
                                list_move_opt.push(mouvement);
                            } 
                            
                            if score_opt < beta {
                                beta = score_opt;
                            }
                            if res <= alpha {
                                break; //on coupe la branche
                            }
                        } 
                    }
                    let randIndex = rand::thread_rng().gen_range(0..list_move_opt.len());
                    //print!("score :{} ", score_opt);
                    (score_opt, Some(list_move_opt[randIndex]))

                    
            }else {
                (state.value(), None) //return value ce node
            }
        }        
        alpha_beta(state, self.0, self.1, self.2).1
    }
    */
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement>{
        
        fn alpha_beta_avec_memoization(state: &Configuration, profondeur : u8, mut alpha: i8, mut beta: i8, memoisation: &mut HashMap<(String, Movement), (i8, Movement)>) -> (i8, Option<Movement>){
            if state.movements().next().is_some() && profondeur > 0 {
                    
                let mut score_opt = i8::MAX;
                let mut list_move_opt: Vec<Movement> = Vec::new();
                let mut val_list_opt = i8::MAX;

                for mouvement in state.movements() {
                    let mut res: i8;
                    //let mut etat = state.play(&mouvement);
                    //score_opt = cmp::max(score_opt, minimax(&etat, profondeur-1, false));
                    if memoisation.contains_key(&(state.serialize(), mouvement)) {
                        res = match memoisation.get(&(state.serialize(), mouvement)){
                            Some(val) => val.0,
                            None => 127,
                        };
                        //print!("\n ICI \n");
                        
                    }else {
                        res = -alpha_beta_avec_memoization(&state.play(&mouvement), profondeur-1, -beta, -alpha, memoisation).0; //pour optimiser ou minimiser selon si on joue ou l'adv
                        memoisation.insert((state.serialize(), mouvement), (res, mouvement));
                    }
                    //print!("AlphaBetaMemo :scoreOpt={}, scoreNouv={}\n", score_opt, res.0);
                    if score_opt > res {
                        //print!("AlphaBetaMemo :scoreOpt={}, scoreNouv={}\n", score_opt, -res.0);
                        score_opt = res;
                        if score_opt == val_list_opt {
                            list_move_opt.push(mouvement);
                        } else{
                            val_list_opt = score_opt;
                            list_move_opt = Vec::new();
                            list_move_opt.push(mouvement);
                        } 
                        
                        if score_opt < beta {
                            beta = score_opt;
                        }
                        if res <= alpha {
                            break; //on coupe la branche
                        }
                    } 
                }
                let randIndex = rand::thread_rng().gen_range(0..list_move_opt.len());
                //print!("score :{} ", score_opt);
                (score_opt, Some(list_move_opt[randIndex]))

                
        }else {
            (state.value(), None) //return value ce node
        }
    }        
        let mut memo: HashMap<(String, Movement), (i8, Movement)> = HashMap::new();
        alpha_beta_avec_memoization(state, self.0, self.1, self.2, &mut memo).1   
    }
}

