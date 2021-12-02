use std::io::BufRead;
use std::io::BufReader;
use std::io; 
use std::fs::File; 
mod day_2;

fn main() {
    // --snip--
    let filename = "input.txt";
    println!("In file {}", filename);


    let lines = file_to_vec(filename.to_string());
    let input = lines.ok().unwrap();

    day_2::__day2(input);
    
    
}


fn file_to_vec(filename: String) -> io::Result<Vec<String>> { 
    let file_in = File::open(filename)?; 
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
} 
