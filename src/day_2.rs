pub fn __day2(input: std::vec::Vec<std::string::String>) { 
    
    let mut count = 0;
    let mut depth = 0;
    
    for x in 0..input.len() {
        let line = &input[x];
        
        let mut iter = line.split_whitespace();
        let word = iter.next().unwrap();
        let num = iter.next().unwrap().parse::<i32>().unwrap();
        if word.eq("forward") {
            count += num;
        } else if word.eq("up"){
            depth -= num;
        } else {
            depth += num;
        }
    }

    println!("{}", count*depth);

} 

pub fn __day2_2(input: std::vec::Vec<std::string::String>) { 
    
    let mut count = 0;
    let mut depth = 0;
    let mut aim = 0;
    
    for x in 0..input.len() {
        let line = &input[x];
        
        let mut iter = line.split_whitespace();
        let word = iter.next().unwrap();
        let num = iter.next().unwrap().parse::<i32>().unwrap();
        if word.eq("forward") {
            count += num;
            depth += aim * num;
        } else if word.eq("up"){
            aim -= num;
        } else {
            aim  += num;
        }   
    }

    println!("{}", count*depth);

} 