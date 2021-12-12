
pub fn __day7(input: std::vec::Vec<std::string::String>) { 

    let list = input[0].split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut pos:[u32;2000] = [0;2000];
    for i in 0..2000 {
        for x in &list {
            if i > *x {
                pos[i as usize] += i - x;
            } else {
                pos[i as usize] += x - i;
            }
        }
    }
    let min = pos.iter().min().unwrap();
    println!("{}", min);

} 

pub fn __day7_2(input: std::vec::Vec<std::string::String>) { 

    let list = input[0].split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut pos:[u32;2000] = [0;2000];
    for i in 0..2000 {
        for x in &list {
            if i > *x {
                pos[i as usize] += (i - x + 1) * (i - x) / 2;
            } else {
                pos[i as usize] += (x - i + 1) * (x - i) / 2;
            }
        }
    }
    let min = pos.iter().min().unwrap();
    println!("{}", min);

} 
                