
pub fn __day11(input: std::vec::Vec<std::string::String>) { 

    let mut count = 0;
    let mut grid : [[u32;10];10] = [[0;10];10];

    for i in 0..10 {
        let line : Vec<char> = input[i].chars().collect();
        for j in 0..10 {
            grid[i][j] = (line[j].to_string()).parse::<u32>().unwrap();
        }
    }

    for __days in 0..100 {
        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
            }
        }
        for i in 0..10 {
            for j in 0..10 {
                if grid[i][j] > 9 {
                    count += flash(&mut grid, i, j);
                }  
            }
        }
    }
    println!("{}", count);
} 

pub fn __day11_2(input: std::vec::Vec<std::string::String>) { 
    
    let mut count = 0;
    let mut grid : [[u32;10];10] = [[0;10];10];

    for i in 0..10 {
        let line : Vec<char> = input[i].chars().collect();
        for j in 0..10 {
            grid[i][j] = (line[j].to_string()).parse::<u32>().unwrap();
        }
    }
    for __days in 0..1000 {
        let mut sync = true;

        for i in 0..10 {
            for j in 0..10 {
                if grid[i][j] != 0 {
                    sync = false;
                }
                grid[i][j] += 1;
            }
        }
        if sync {
            count = __days;
            break;
        }
        for i in 0..10 {
            for j in 0..10 {
                if grid[i][j] > 9 {
                    count += flash(&mut grid, i, j);
                }  
            }
        }
    }

    println!("{}", count);
} 

pub fn flash(grid: &mut [[u32; 10]; 10], i : usize, j : usize) -> u32 { 
    if i==10 || j==10 || grid[i][j] == 0 {
        return 0;
    }
    grid[i][j] += 1;
    if grid[i][j]  > 9 {
        grid[i][j] = 0;
        1 + if i != 0 { flash(grid, i-1, j) + flash(grid, i-1, j+1)} else {0}
          + if j != 0 { flash(grid, i, j-1) + flash(grid, i+1, j-1) } else {0}
          + if i != 0 && j != 0 {flash(grid, i-1, j-1)} else {0}
          + flash(grid, i, j+1)
          + flash(grid, i+1, j)
          + flash(grid, i+1, j+1)
    } else {
        0
    }
    
}