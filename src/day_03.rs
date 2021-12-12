use std::collections::VecDeque;

pub fn __day3(input: std::vec::Vec<std::string::String>) { 
    
    let mut count = vec![0,0,0,0,0,0,0,0,0,0,0,0];
    
    for x in 0..input.len() {
        let line = &input[x];
        
        let iter = line.chars();
        let mut counter = 0;
        for y in iter {
            if y.eq(&'1') {
                count[counter] += 1;
            }
            counter += 1;
        }
    }
    
    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    for i in 0..count.len() {
        if count[i]>input.len()/2 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    println!("{}", isize::from_str_radix(&gamma, 2).unwrap()  * isize::from_str_radix(&epsilon, 2).unwrap());

} 

pub fn __day3_2(input: std::vec::Vec<std::string::String>) { 
    

    let mut oxyg = VecDeque::from_iter(&input);
    let mut co = VecDeque::from_iter(&input);
    
    let mut stroxy = String::from("");
    let mut strco = String::from("");


    for i in 0..12 {
        let mut itera = 0;
        let mut counter = 0;
        for x in &oxyg {
            
            let mut iter = x.chars();
            for __j in 0..i {
                iter.next();
            }
            if iter.next().unwrap().eq(&'1') {
                counter += 1;
            }
        }

        let bitoxy = (counter as f32) >= (oxyg.len() as f32 / 2.0);
        if bitoxy {
            stroxy.push('1');
        } else {
            stroxy.push('0');
        }
        loop {
            if oxyg.len() == 1 || itera >= oxyg.len() {
                break;
            }
            let line = &oxyg[itera];
            
            let mut iter = line.chars();
            for __j in 0..i {
                iter.next();
            }
            let ch = iter.next();
            let mut minus = false;
            if ch.unwrap().eq(&'0') && bitoxy || ch.unwrap().eq(&'1') && !bitoxy {
                oxyg.remove(itera);
                minus = true;
            } 
            itera += 1;
            if minus {  
                itera -= 1;
            }
        }
        
        
    } 

    for i in 0..12 {
        let mut itera = 0;
        let mut counter = 0;
        for x in &co {
            
            let mut iter = x.chars();
            for __j in 0..i {
                iter.next();
            }
            if iter.next().unwrap().eq(&'1') {
                counter += 1;
            }
        }

        let bitco = (counter as f32) >= (co.len() as f32 / 2.0);
        if bitco {
            strco.push('0');
        } else {
            strco.push('1');
        }
        loop {
            if co.len() == 1 || itera >= co.len() {
                break;
            }
            let line = &co[itera];
            
            let mut iter = line.chars();
            for __j in 0..i {
                iter.next();
            }
            let ch = iter.next().unwrap();
            let mut minus = false;
            if ch.eq(&'1') && bitco || ch.eq(&'0') && !bitco {
                co.remove(itera);
                minus = true;
            }
            itera += 1;
            if minus {
                itera -= 1;
            }
        }
    } 

    
    println!("{} * {} = {}", oxyg[0], co[0], isize::from_str_radix(oxyg[0], 2).unwrap()  * isize::from_str_radix(co[0], 2).unwrap());
}