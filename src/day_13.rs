
pub fn __day13_2(input: std::vec::Vec<std::string::String>) { 
    
    let mut grid: Vec<(u32, u32)> = Vec::new();
    let mut first = true;
    let mut next = false;
    for x in input {
        if x.len() < 1 {
            next = true;
        } else if next {
            let fold : Vec<&str> = x.split("=").map(|s| s).collect();
            let edge = fold[1].parse::<u32>().unwrap();
            if fold[0].split(" ").map(|s| s).collect::<Vec<&str>>()[2].eq("x") {
                for i in 0..grid.len(){
                    while grid[i].0 == edge {
                        grid.remove(i);
                    }
                    if grid[i].0 > edge {
                        grid[i].0 -= (grid[i].0 - edge)*2;
                    }
                }
            } else {
                for i in 0..grid.len(){
                    while grid[i].1 == edge {
                        grid.remove(i);
                    }
                    if grid[i].1 > edge {
                        grid[i].1 -= (grid[i].1 - edge)*2;
                    }
                }
            }
            if first { println!("Part 1: {}", grid.len()); first = false; }
        } else {
            let num = x.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            grid.push((num[0], num[1]));
        }
    }
    let mut arr : [[&str;10];50] = [[".";10];50];
    for x in grid {
        arr[x.0 as usize][x.1 as usize] = "#";
    }

    for j in 0..10 {
        for i in 0..50 {
            print!("{}", arr[i][j]);
        }
        println!();
    }
} 
