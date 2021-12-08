use std::time::SystemTime;
use std::io::BufRead;
use std::io::BufReader;
use std::io; 
use std::fs::File; 
mod day_8   ;

fn main() {
    let now = SystemTime::now();
    let filename = "/home/lukas/Documents/VSC/AoC/AoC/target/debug/input.txt";
    println!("In file {}", filename);

    let input = file_to_vec(filename.to_string()).ok().unwrap();
    day_8::__day8(input);
    println!("Task 2:");
    let input2 = file_to_vec(filename.to_string()).ok().unwrap();
    day_8::__day8_2(input2);
    
    let time = now.elapsed().unwrap();
    println!("Time: {}", time.as_micros());
}


fn file_to_vec(filename: String) -> io::Result<Vec<String>> {  
    Ok(BufReader::new(File::open(filename)?).lines().filter_map(io::Result::ok).collect()) 
}
