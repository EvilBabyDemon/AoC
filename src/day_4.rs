
pub fn __day4(input: std::vec::Vec<std::string::String>) { 
    
    
    let mut bingo:[[[u32;5];5];100] = [[[0;5];5];100];
    let mut hits:[[[bool;5];5];100] = [[[false;5];5];100];
    
    let numbers = &input[0].split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();



    for i in (2..input.len()).step_by(6) {
        for j in 0..5 {
            let line = &input[i+j];
            let row = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            for k in 0..5 {
                bingo[(i-2)/6][j][k] = row[k];
            }
        }
    }
    
    let mut lastnum = 0;
    let mut grid = 0;

    'outer: for x in numbers {
        for i in 0..100{
            for j in 0..5{
                for k in 0..5{
                    if bingo[i][j][k] == *x {
                        hits[i][j][k] = true;
                        let mut done_hor = true;
                        let mut done_ver = true;
                        for y in 0..5{
                            if !hits[i][y][k] {
                                done_hor = false;
                            }
                            if !hits[i][j][y] {
                                done_ver = false;
                            }
                        }

                        if done_hor || done_ver {
                            lastnum = *x;
                            grid = i;
                            break 'outer;
                        }
                    }
                }
            }    
        }
    }

    let mut sum = 0;
    for i in 0..5{
        for j in 0..5{
            if !hits[grid][i][j] {
                sum += bingo[grid][i][j];
            } 
        }
    }
    


    println!("{}", lastnum*sum);

} 

pub fn __day4_2(input: std::vec::Vec<std::string::String>) { 
    
      
    
    let mut bingo:[[[u32;5];5];100] = [[[0;5];5];100];
    let mut hits:[[[bool;5];5];100] = [[[false;5];5];100];
    let mut win:[bool;100] = [false; 100];
    let numbers = &input[0].split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();



    for i in (2..input.len()).step_by(6) {
        for j in 0..5 {
            let line = &input[i+j];
            let row = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            for k in 0..5 {
                bingo[(i-2)/6][j][k] = row[k];
            }
        }
    }
    
    let mut lastnum = 0;
    let mut grid = 0;

    'outer: for x in numbers {
        for i in 0..100{
            for j in 0..5{
                for k in 0..5{
                    if bingo[i][j][k] == *x {
                        if win[i] {
                            continue;
                        }
                        hits[i][j][k] = true;
                        let mut done_hor = true;
                        let mut done_ver = true;
                        for y in 0..5{
                            if !hits[i][y][k] {
                                done_hor = false;
                            }
                            if !hits[i][j][y] {
                                done_ver = false;
                            }
                        }

                        if done_hor || done_ver {
                            win[i] = true;
                            let mut last = true;
                            for y in 0..100{
                                if !win[y] {
                                    last = false;
                                }
                            }
                            if last {
                                lastnum = *x;
                                grid = i;
                                break 'outer;
                            }
                        }
                    }
                }
            }    
        }
    }

    let mut sum = 0;
    for i in 0..5{
        for j in 0..5{
            if !hits[grid][i][j] {
                sum += bingo[grid][i][j];
            } 
        }
    }

    println!("{}", lastnum*sum);

    
} 