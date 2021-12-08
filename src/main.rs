use std::io::BufRead;
use std::io::BufReader;
use std::io; 
use std::fs::File; 
mod day_9;

fn main() {
    
    let filename = "input.txt";
    println!("In file {}", filename);

    let input = file_to_vec(filename.to_string()).ok().unwrap();
    day_9::__day9(input);
    println!("Task 2:");
    let input2 = file_to_vec(filename.to_string()).ok().unwrap();
    day_9::__day9_2(input2);
    
}


fn file_to_vec(filename: String) -> io::Result<Vec<String>> { 
    let file_in = File::open(filename)?; 
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
} 
