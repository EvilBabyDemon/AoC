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
            if !unique[0].contains(x) {
                let y : Vec<char> = x.chars().collect();
                chars[0] = y[0];
            }
        }

        //b and d
        let four = unique[2];
        let one: Vec<char>= unique[0].chars().collect();
        let four = &four.replace(&[one[0], one[1]][..], "");
        let four_ch : Vec<char>= four.chars().collect();
        for i in 6..9 {
            if unique[i].contains(four_ch[0]) && unique[i].contains(four_ch[1]) {
                continue;
            }
            let con = unique[i].contains(four_ch[0]); 
            chars[1] = if con {four_ch[0]} else {four_ch[1]};
            chars[3] = if con {four_ch[1]} else {four_ch[0]};
            break;
        }

        //c and f
        for i in 6..9 {
            if unique[i].contains(one[0]) && unique[i].contains(one[1]) {
                continue;
            }
            let con = unique[i].contains(one[0]);
            chars[5] = if con {one[0]} else {one[1]};
            chars[2] = if con {one[1]} else {one[0]};
            break;
        }
        
        //g and e
        let mut fives : [&str;3] = [&unique[3].replace(&[chars[0], chars[1], chars[2], chars[3], chars[5]][..], ""),
        &unique[4].replace(&[chars[0], chars[1], chars[2], chars[3], chars[5]][..], ""),
        &unique[5].replace(&[chars[0], chars[1], chars[2], chars[3], chars[5]][..], "")];
        fives.sort_by(|a, b| a.len().cmp(&b.len()));
        chars[6] = fives[0].chars().collect::<Vec<char>>()[0];
        chars[4] = fives[2].replace(chars[6], "").chars().collect::<Vec<char>>()[0];
        
        let mut intnum : [u32;4] = [0;4];
        
        for i in 0..4 {
            intnum[i] = match number[i].len(){
                2=> 1,
                3=> 7,
                4=> 4,
                5=> 
                    if !number[i].contains(chars[2]){
                        5
                    } else if !number[i].contains(chars[5]){
                        2
                    } else {
                        3
                    }
                , 
                6=> 
                    if !number[i].contains(chars[2]){
                        6
                    } else if !number[i].contains(chars[3]){
                        0
                    } else {
                        9
                    }
                ,
                7=> 8,
                _=> 10,
            }
        } 
        counter += intnum[0] * 1000 + intnum[1] * 100 + intnum[2] * 10 + intnum[3];
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
