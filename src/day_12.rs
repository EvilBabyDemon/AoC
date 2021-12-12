use std::collections::HashMap;
use std::collections::HashSet;

pub fn __day12_2(input: std::vec::Vec<std::string::String>) { 
    
    let mut nodes : HashMap<&str, std::vec::Vec<&str>> = HashMap::new();

    for i in 0..input.len() {
        let line : Vec<&str> = input[i].split("-").collect();
        nodes.entry(line[0]).or_insert(Vec::new()).push(line[1]);
        nodes.entry(line[1]).or_insert(Vec::new()).push(line[0]);
    }
    let mut visited : std::collections::HashSet<&str> = HashSet::new();
    visited.insert("start");
    
    println!("Part 1: {}", paths(&nodes, &visited, "start", true));
    println!("Part 2: {}", paths(&nodes, &visited, "start", false));
}

fn paths (nodes: &HashMap<&str, std::vec::Vec<&str>>,  visited : &std::collections::HashSet<&str>, atm : &str, twice : bool) -> u32 {
    if atm.eq("end") {
        return 1;
    }
    let mut count = 0; 
    for x in nodes.get(atm) {
        for i in 0..x.len() {
            if !visited.contains(x[i]) {
                let mut visit = visited.clone();
                if !x[i].chars().next().unwrap().is_uppercase() {
                    visit.insert(x[i]);
                }
                count += paths(&nodes, &visit, x[i], twice);
            } else if !twice && !x[i].eq("start") {
                count += paths(&nodes, &visited, x[i], true);
            }
        }
    }
    return count;
}