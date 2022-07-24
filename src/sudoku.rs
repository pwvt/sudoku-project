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

// digits   = "123456789";
// rows     = "ABCDEFGHI";

pub fn solve(board: &str) -> Vec<String> {
    todo!("Implement me");
}