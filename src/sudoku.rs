use rayon::{iter::*, str::ParallelString};
use rayon::prelude::*;

fn cross(A: &str, B: &str) -> Vec<String> {
    A.par_chars()
        .flat_map(|a| 
            B.par_chars()
                .map(|b| format!("{}{}", a, b))
                .collect::<Vec<String>>()
        )
        .collect::<Vec<String>>()
}

pub fn solve(board: &str) -> Vec<String> {
    let cols = String::from("123456789");
    let rows = String::from("ABCDEFGHI");
    let squares = cross(&rows, &cols);
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
    
    // println!("{:?}", unit_list);


    todo!("Implement me");
}