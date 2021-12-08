
pub fn __day8(input: std::vec::Vec<std::string::String>) { 

    let mut counter = 0;
    for i in 0..input.len() {
        let line = input[i].split("|").collect::<Vec<&str>>();
        let number = line[1].split_whitespace().collect::<Vec<&str>>();

        for x in number {
            if x.len() > 1 && x.len() < 5 || x.len() == 7{
                counter += 1;
            } 
        }
    }
    
    println!("{}", counter);

} 

pub fn __day8_2(input: std::vec::Vec<std::string::String>) { 

    let mut counter = 0;
    for i in 0..input.len() {
        let line = input[i].split("|").collect::<Vec<&str>>();
        let mut unique = line[0].split_whitespace().collect::<Vec<&str>>();
        let number = line[1].split_whitespace().collect::<Vec<&str>>();

        unique.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut chars:[char;7] = [' ';7];

        //a
        for x in unique[1].split("") {
            if unique[0].contains(x) {
                continue;
            }
            let y : Vec<char>= x.chars().collect();
            chars[0] = y[0];
        }

        //b and d
        let four = unique[2];
        let one: Vec<char>= unique[0].chars().collect();
        let four = &four.replace(&[one[0], one[1]][..], "");

        for i in 6..9 {
            let y : Vec<char>= four.chars().collect();
            if unique[i].contains(y[0]) && unique[i].contains(y[1]) {
                continue;
            }
            if unique[i].contains(y[0]) {
                chars[1] = y[0];
                chars[3] = y[1];
            } else {
                chars[1] = y[1];
                chars[3] = y[0];
            }
            break;
        }
        //c and f
        for i in 6..9 {
            if unique[i].contains(one[0]) && unique[i].contains(one[1]) {
                continue;
            }
            if unique[i].contains(one[0]) {
                chars[5] = one[0];
                chars[2] = one[1];
            } else {
                chars[5] = one[1];
                chars[2] = one[0];
            }
            break;
        }
        //g and e
        let five0 = unique[3];
        let five1 = unique[4];
        let five2 = unique[5];

        let five0 = &five0.replace(&[chars[0], chars[1], chars[2], chars[3], chars[5]][..], "");
        let five1 = &five1.replace(&[chars[0], chars[1], chars[2], chars[3], chars[5]][..], "");
        let five2 = &five2.replace(&[chars[0], chars[1], chars[2], chars[3], chars[5]][..], "");
        
        if five0.len() == 2 {
            let g : Vec<char>= five1.chars().collect();
            let five0 = &five0.replace(g[0], "");
            let e : Vec<char>= five0.chars().collect();
            chars[6] = g[0];
            chars[4] = e[0];
        } else if five2.len() == 2 {
            let g : Vec<char>= five1.chars().collect();
            let five2 = &five2.replace(g[0], "");
            let e : Vec<char>= five2.chars().collect();
            chars[6] = g[0];
            chars[4] = e[0];
        } else {
            let g : Vec<char>= five0.chars().collect();
            let five1 = &five1.replace(g[0], "");
            let e : Vec<char>= five1.chars().collect();
            chars[6] = g[0];
            chars[4] = e[0];
        }
    
        
        let mut intnum : [u32;4] = [0;4];
        
        for i in 0..4 {
            match number[i].len(){
                2=> intnum[i] = 1,
                3=> intnum[i] = 7,
                4=> intnum[i] = 4,
                5=> 
                    if !number[i].contains(chars[2]){
                        intnum[i] = 5;
                    } else if !number[i].contains(chars[5]){
                        intnum[i] = 2;
                    } else {
                        intnum[i] = 3;
                    }
                , 
                6=> 
                    if !number[i].contains(chars[2]){
                        intnum[i] = 6;
                    } else if !number[i].contains(chars[3]){
                        intnum[i] = 0;
                    } else {
                        intnum[i] = 9;
                    }
                ,
                7=> intnum[i] = 8,
                _=> print!("error"),
            }
        } 
        let num =  intnum[0] * 1000 + intnum[1] * 100 + intnum[2] * 10 + intnum[3];
        println!("{}", num);
        counter += num;
    }
    
    println!("{}", counter);

} 

/*Check for len 3 and 2 
figure out a

Check for len 4 and 6
Remove len 4 len 2
Check if both b and d are in, till no
Which one is in -> b the one not d

Check for len 2 and 6
Check if both c and f are in, till no
Which one is in -> f the one not c


Check for len 5
Check for one with e and g and one with only one
Which one is in -> g the one not e
*/
