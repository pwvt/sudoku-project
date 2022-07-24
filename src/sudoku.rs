use std::ops::Index;

use rayon::{iter::*, str::ParallelString};
use rayon::prelude::*;
use bit_set::BitSet;
use chashmap::CHashMap;

fn cross(A: &str, B: &str) -> Vec<String> {
    A.par_chars()
        .flat_map(|a| 
            B.par_chars()
                .map(|b| format!("{}{}", a, b))
                .collect::<Vec<String>>()
        )
        .collect::<Vec<String>>()
}

fn make_unit_list(cols: &str, rows: &str) -> Vec<Vec<String>> {
    let mut unit_list: Vec<Vec<String>> = vec![];
    unit_list.extend(
        cols.par_chars()
            .map(|c| cross(&rows, &c.to_string()))
            .collect::<Vec<Vec<String>>>()
    );
    unit_list.extend(
        rows.par_chars()
            .map(|r| cross(&cols, &r.to_string()))
            .collect::<Vec<Vec<String>>>()
    );
    unit_list.extend(
        vec!["ABC", "DEF", "GHI"].par_iter()
            .flat_map(|rg| 
                vec!["123","456","789"].par_iter()
                    .map(|&cg|cross(rg, cg))
                    .collect::<Vec<Vec<String>>>()
                )
            .collect::<Vec<Vec<String>>>()
    );
    unit_list
}

fn eliminate() {
    todo!("Implement me");
}

fn assign(values: &CHashMap<String, BitSet>, s: &str, d: &BitSet) {

    todo!("Implement me");
}

fn parse_grid(squares: &Vec<String>, board: &str) -> CHashMap<String, BitSet> {
    let values: CHashMap<String, BitSet> = CHashMap::new();
    squares.par_iter().for_each(|sq|{
            values.insert(sq.clone(), BitSet::from_iter((0..=9).into_iter()));
        }
    );
    let default = board_values(squares, board).into_iter().collect::<Vec<(String, BitSet)>>();
    default.par_iter().for_each(|(s, d)|assign(&values, s, d));
    todo!("Implement me");
}

fn board_values(squares: &Vec<String>, board: &str) -> CHashMap<String, BitSet> {
    let default: CHashMap<String, BitSet> = CHashMap::new();
    squares.par_iter()
        .zip(board.par_chars().map(|c| {
                let num = c.to_string().parse::<u8>();
                if num.is_ok() {BitSet::from_bytes(&[num.unwrap()])} 
                else { BitSet::new() } 
            }).collect::<Vec<BitSet>>()
        )
        .for_each(|(sq, bs)| {
                default.insert(sq.clone(), bs);
            }
        );
    default
    // println!("{:?}", default);
}


pub fn solve(board: &str) -> Vec<String> {
    let digits = String::from("123456789");
    let rows = String::from("ABCDEFGHI");
    let squares = cross(&rows, &digits);
    let unit_list = make_unit_list(&digits, &rows);
    board_values(&squares, board);
    // println!("{:?}", unit_list);


    todo!("Implement me");
}