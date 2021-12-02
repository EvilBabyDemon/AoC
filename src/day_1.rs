use std::io::BufRead;
use std::io::BufReader;
use std::fs::File; 

fn main() {
    // --snip--
    let filename = "input.txt";
    println!("In file {}", filename);

    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);

    let mut first = true;
    let mut second = true;

    let mut old : i32 = 0;
    let mut middle : i32 = 0;
    let mut newer : i32 = 0;
    let mut count = 0;

    for line in reader.lines() {
        
        let number = &line.unwrap().parse::<i32>().unwrap();
        if first {
            old += number;
            first = false;
            continue;
        }
        
        if second {
            middle = number;
            second = false;
            continue;
        }
        
        if number > old {
            count += 1;
        }
        
        old = middle;
        middle = newer;
        newer = *number;

    }
    println!("{}", count);

    
}
