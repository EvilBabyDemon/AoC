
pub fn __day9(input: std::vec::Vec<std::string::String>) { 

    let mut grid:[[u32;100];100] = [[0;100];100];

    for i in 0..100 {
        let line : Vec<char> = input[i].chars().collect();
        for j in 0..100 {
            grid[i][j] = (line[j].to_string()).parse::<u32>().unwrap();
        }
    }

    let mut count = 0;
    for i in 0..100 {
        for j in 0..100 {
            if i != 0 && grid[i][j] >= grid[i-1][j] || j != 99 && grid[i][j] >= grid[i][j+1] || i != 99 && grid[i][j] >= grid[i+1][j] || j != 0 && grid[i][j] >= grid[i][j-1] {
                continue;
            }
            count += grid[i][j] + 1;
        }
    }

    
    println!("{}", count);

} 

pub fn __day9_2(input: std::vec::Vec<std::string::String>) { 

    let mut grid:[[u32;100];100] = [[0;100];100];

    for i in 0..100 {
        let line : Vec<char> = input[i].chars().collect();
        for j in 0..100 {
            grid[i][j] = (line[j].to_string()).parse::<u32>().unwrap();
        }
    }
    let mut basin:[u32;3] = [0;3];
    for i in 0..100 {
        for j in 0..100 {
            if i != 0 && grid[i][j] >= grid[i-1][j] || j != 99 && grid[i][j] >= grid[i][j+1] || i != 99 && grid[i][j] >= grid[i+1][j] || j != 0 && grid[i][j] >= grid[i][j-1] {
                continue;
            }
            let count = counting(&mut grid, i, j);
            if count > basin[0]{
                basin[0] = count;
            }
            basin.sort();
        }
    }
    println!("{}", basin[0]*basin[1]*basin[2]);
} 



pub fn counting(grid: &mut [[u32; 100]; 100], i : usize, j : usize) -> u32 { 
    if i==100 || j==100 || grid[i][j] == 9 {
        return 0;
    }
    grid[i][j] = 9;
    
    1 + if i != 0 { counting(grid, i-1, j) } else {0}
      + if j != 0 { counting(grid, i, j-1) } else {0}
      + counting(grid, i, j+1)
      + counting(grid, i+1, j)
}

use rayon::prelude::*;

#[macro_export]
macro_rules! oob {
    { $x:expr, $i:expr, $j:expr } => {
        //let i = $y as usize;
        //let j = $z as usize;
        $x[$i][$j] > 5 || $i != 0 && $x[$i][$j] >= $x[$i-1][$j] || $j != 99 && $x[$i][$j] >= $x[$i][$j+1] || $i != 99 && $x[$i][$j] >= $x[$i+1][$j] || $j != 0 && $x[$i][$j] >= $x[$i][$j-1]
    };
}


pub fn __day9_3(input: std::vec::Vec<std::string::String>) { 

    let mut grid:[[u32;100];100] = [[0;100];100];

    for i in 0..100 {
        let line : Vec<char> = input[i].chars().collect();
        for j in 0..100 {
            grid[i][j] = (line[j].to_string()).parse::<u32>().unwrap();
        }
    }

    let mut min : Vec<[u32;3]> = Vec::new(); 

    for i in 0..100 {
        for j in 0..100 {
            if grid[i][j] > 5 || i != 0 && grid[i][j] >= grid[i-1][j] || j != 99 && grid[i][j] >= grid[i][j+1] || i != 99 && grid[i][j] >= grid[i+1][j] || j != 0 && grid[i][j] >= grid[i][j-1] {
                continue;
            }
            min.push([0, i as u32, j as u32]);
        }
    }
    
    //counting(&mut grid, i.0, j.0))
    //grid.par_iter_mut().enumerate().for_each(|i|  min.push(i.1.par_iter_mut().enumerate().for_each(|j| if oob!(grid, i.0, j.0) {counting(&mut grid, i.0, j.0)} else {0}).filter_map(|p| match p {0 => None, _ => p}).collect()));
    //let mut out : Vec<u32> = 0;
    min.par_iter_mut().for_each(|x| x[0] = counting(&mut grid.clone(), x[1] as usize, x[2] as usize));
    

    min.par_sort_by(|a, b| b[0].cmp(&a[0]));
    println!("{}", min[0][0]*min[1][0]*min[2][0]);

}

