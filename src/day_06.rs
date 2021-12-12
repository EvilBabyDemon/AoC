
pub fn __day6(input: std::vec::Vec<std::string::String>) { 

    let mut list = input[0].split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    for __i in 0..80 {
        let size = list.len();
        for j in 0..size {
            if list[j] == 0{
                list.push(8);
                list[j] = 6;
            } else {
                list[j] -= 1;
            }
        }
    }

    println!("{}", list.len());

} 

pub fn __day6_2(input: std::vec::Vec<std::string::String>) { 

    let list = input[0].split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    
    let mut fish:[u64;9] = [0;9];

    for x in list {
        fish[x as usize] += 1;
    }

    for __i in 0..256 {
        let mut fish2:[u64;9] = [0;9];
        for j in 0..9 {
            if j == 0{
                fish2[8] = fish2[j];
                fish2[6] = fish2[j]; 
            } else {
                fish2[j-1] = fish[j]; 
            }
        }
        for j in 0..9 {
            fish[j] = fish2[j];
        }
    }

    let mut count = 0;
    for x in fish {
        count += x;
    }

    println!("{}", count);

} 
                