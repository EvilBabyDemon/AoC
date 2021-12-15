
pub fn __day15(input: std::vec::Vec<std::string::String>) { 

    let mut grid = Vec::new();
    for x in input {
        let line = x.chars().map(|s| s.to_string().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        grid.push(line);
    }

    let mut dp = grid.clone();
    dp[0][0]=0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if j==0 && i==0 {
                continue;
            } else if i == 0 {
                dp[i][j] += dp[i][j-1];
            } else if j == 0 {
                dp[i][j] += dp[i-1][j];
            } else {
                if dp[i][j-1] > dp[i-1][j] {
                    dp[i][j] += dp[i-1][j];
                } else {
                    dp[i][j] += dp[i][j-1];
                }
            }
        }
    }
    println!("{}", dp[dp.len()-1][dp[0].len()-1]);
}




pub fn __day15_2(input: std::vec::Vec<std::string::String>) { 
    
    print!("{}", input[0]);
}
    
 
