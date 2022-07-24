use rayon::{iter::*, str::ParallelString};

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
    // println!("{:?}", squares);
    let mut unit_list: Vec<Vec<String>> = vec![];
    let temp = cols.par_chars().map(|c| cross(&rows, &c.to_string())).collect::<Vec<Vec<String>>>().into_iter();
    unit_list.extend(
        cols.par_chars()
            .map(|c| cross(&rows, &c.to_string()))
            .collect::<Vec<Vec<String>>>()
            .into_iter()
        );
    
    // unitlist = ([cross(rows, c) for c in cols] +
    //         [cross(r, cols) for r in rows] +
    //         [cross(rs, cs) for rs in ('ABC','DEF','GHI') for cs in ('123','456','789')])


    todo!("Implement me");
}