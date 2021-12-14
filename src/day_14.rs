

use std::collections::HashMap;

pub fn __day14(input: std::vec::Vec<std::string::String>) { 

    let start = input.get(0).unwrap();
    let mut rules : HashMap<&str, char> = HashMap::new();
    let mut uses : HashMap<char, u32> = HashMap::new();
    
    for i in 2..input.len() {
        let line = input[i].split(" -> ").map(|s| s).collect::<Vec<&str>>();
        rules.insert(line[0], line[1].chars().next().unwrap());
    }
    let mut old : char = ' ';
    for x in start.chars() {
        let u = uses.entry(x).or_default();
        *u += 1;
        if !old.eq(&' ') {
            let word = old.to_string() + &x.to_string();
            insertion(&word, &rules, &mut uses, 9);
        }
        old = x;
    }
    
    let mut nums: Vec<u32> = Vec::new(); 
    for x in uses {
       nums.push(x.1);
    }

    nums.sort();

    println!("{}", nums[nums.len()-1] - nums[0]);
}

pub fn insertion(word : &str, rules: &HashMap<&str, char>, uses: &mut HashMap<char, u32>, iter : u32) {

    let result = rules.get(word).unwrap();

    let u = uses.entry(*result).or_default();
    *u += 1;
    let mut cha = word.chars();
    let first = cha.next().unwrap().to_string() + &result.to_string();
    let second = result.to_string() + &cha.next().unwrap().to_string();
    if iter == 0 {
        return;
    }

    insertion(&first, rules, uses, iter - 1);
    insertion(&second, rules, uses, iter - 1);
}   




pub fn __day14_2(input: std::vec::Vec<std::string::String>) { 
    
    let start = input.get(0).unwrap();
    let mut rules : HashMap<&str, char> = HashMap::new();
    let mut uses : HashMap<char, u32> = HashMap::new();
    
    for i in 2..input.len() {
        let line = input[i].split(" -> ").map(|s| s).collect::<Vec<&str>>();
        rules.insert(line[0], line[1].chars().next().unwrap());
    }

    

    let mut old : char = ' ';
    for x in start.chars() {
        let u = uses.entry(x).or_default();
        *u += 1;
        if !old.eq(&' ') {
            let word = old.to_string() + &x.to_string();
            insertion(&word, &rules, &mut uses, 39);
        }
        old = x;
    }
    
    let mut nums: Vec<u32> = Vec::new(); 
    for x in uses {
       nums.push(x.1);
    }

    nums.sort();

    println!("{}", nums[nums.len()-1] - nums[0]);
} 
