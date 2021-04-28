extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{Greedy, Human, MinMax, AlphaBeta};
use std::fs::File;
use std::io::prelude::*;


fn main() {
    //let board = Board::load("x").expect("failed loading board");
    

    let mut file = File::create("AlphaBetaMemo1_8VS_Same.txt").expect("failed");
    

    for profondeur_rouge in 1..8 {
        //for profondeur_bleu in 1..4 {
            for tour in 0..5 {
                let board = Default::default();
                let mut game = Configuration::new(&board);
                let mut res = game.battle(AlphaBeta(profondeur_rouge, -127, 127), AlphaBeta(profondeur_rouge, -127, 127)); //Rouge - Bleu

                let mut string = String::new();
                string.push_str("Rouge AlphaBetaMemo");
                string.push_str(&format!("-{}", profondeur_rouge));
    
                string.push_str(" vs AlphaBeta : ");
                string.push_str(&format!("-{} : ", profondeur_rouge));
                string.push_str(&res);
                string.push_str("\n");
                file.write_all(string.as_bytes()).expect("failed");
            //}
        }      
    } 
} 
