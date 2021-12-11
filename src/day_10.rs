use std::collections::LinkedList;

pub fn __day10(input: std::vec::Vec<std::string::String>) { 

    let mut list: LinkedList<char> = LinkedList::new();
    let mut count = 0;

    for x in input {
        let line : Vec<char> = x.chars().collect();
        'inner: for y in line {
            if y.eq(&'(') || y.eq(&'[') || y.eq(&'{') || y.eq(&'<') {
                list.push_front(y);
            } else {
                let close = list.pop_front();

                let hit = match close.unwrap() {
                    '(' => y.eq(&')'),
                    '[' => y.eq(&']'),
                    '{' => y.eq(&'}'),
                    '<' => y.eq(&'>'),
                    _ => false,
                };

                if hit {
                    continue;
                } 

               count += match y {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                };
                break 'inner;
            }
        }
    }

    println!("{}", count);

} 

pub fn __day10_2(input: std::vec::Vec<std::string::String>) { 

    let mut counters = Vec::new();

    'outer: for x in input {
        let line : Vec<char> = x.chars().collect();
        
        let mut list: LinkedList<char> = LinkedList::new();
        for y in line {
            if y.eq(&'(') || y.eq(&'[') || y.eq(&'{') || y.eq(&'<') {
                list.push_front(y);
            } else {
                let close = list.pop_front();
                
                let hit = match close.unwrap() {
                    '(' => y.eq(&')'),
                    '[' => y.eq(&']'),
                    '{' => y.eq(&'}'),
                    '<' => y.eq(&'>'),
                    _ => false,
                };

                if hit {
                    continue;
                } 
                continue 'outer;
            }
        }
        let mut count : u64 = 0;
        for y in list {
            count *= 5;
            count += match y {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            };
        }
        counters.push(count);
    }
    counters.sort();

    println!("{}, with len {}", counters[counters.len()/2], counters.len());
} 

