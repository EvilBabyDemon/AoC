use std::collections::HashMap;

pub fn __day5(input: std::vec::Vec<std::string::String>) { 
    
    let mut gridx:std::collections::HashMap<u32, std::collections::HashMap<u32, u32>> = HashMap::new();

    for i in 0..input.len() {
        let line = &input[i];
        let row = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        if row[0] == row[2] {
            let mut first = row[1];
            let mut second = row[3];
            if first > second {
                first = second;
                second = row[1];
            }

            for j in first..second+1 {
                
                let gridy = gridx.entry(row[0]).or_default();
                
                let counter = gridy.entry(j).or_insert(0);
                *counter += 1
            }
        } else if row[1] == row[3] {
            let mut first = row[0];
            let mut second = row[2];
            if first > second {
                first = second;
                second = row[0];
            }
            for j in first..second+1 {
                let gridy = gridx.entry(j).or_default();
                
                let counter = gridy.entry(row[1]).or_insert(0);
                *counter += 1
            }
        } 

    }
    let mut count = 0;
    
    for x in gridx.values() {
        for y in x.values() {
            if *y > 1 {
                count += 1;
            }
        }
    }
    


    println!("{}", count);

} 

pub fn __day5_2(input: std::vec::Vec<std::string::String>) { 
    
      
    
    let mut gridx:std::collections::HashMap<u32, std::collections::HashMap<u32, u32>> = HashMap::new();

    for i in 0..input.len() {
        let line = &input[i];
        let row = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        if row[0] == row[2] {
            let mut first = row[1];
            let mut second = row[3];
            if first > second {
                first = second;
                second = row[1];
            }

            for j in first..second+1 {
                
                let gridy = gridx.entry(row[0]).or_default();
                
                let counter = gridy.entry(j).or_insert(0);
                *counter += 1
            }
        } else if row[1] == row[3] {
            let mut first = row[0];
            let mut second = row[2];
            if first > second {
                first = second;
                second = row[0];
            }
            for j in first..second+1 {
                let gridy = gridx.entry(j).or_default();
                
                let counter = gridy.entry(row[1]).or_insert(0);
                *counter += 1
            }
        } else if row[1] > row[3] && row[0] > row[2] || row[1] < row[3] && row[0] < row[2] {
            let mut firstx = row[0];
            let mut secondx = row[2];
            let mut firsty = row[1];
            if firstx > secondx {
                firstx = secondx;
                secondx = row[0];
                firsty = row[3];
            }

            for j in 0..secondx+1-firstx {
                
                let gridy = gridx.entry(firstx+j).or_default();
                
                let counter = gridy.entry(firsty+j).or_insert(0);
                *counter += 1
            }
        }  else {
            let mut firstx = row[0];
            let mut secondx = row[2];
            let mut y = row[1];
            if firstx > secondx {
                firstx = secondx;
                secondx = row[0];
                y = row[3];
            }

            for j in 0..secondx+1-firstx {
                
                let gridy = gridx.entry(firstx+j).or_default();
                
                let counter = gridy.entry(y-j).or_insert(0);
                *counter += 1
            }
        }

    }
    let mut count = 0;
    
    for x in gridx.values() {
        for y in x.values() {
            if *y > 1 {
                count += 1;
            }
        }
    }
    


    println!("{}", count);
    
} 
                