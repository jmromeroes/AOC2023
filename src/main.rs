use std::{collections::{HashMap, HashSet}, cmp::Ordering, thread::current, mem::swap};

use nom::{bytes::complete::{tag, take_while, take_while1, tag_no_case}, IResult, branch::alt, sequence::{preceded, delimited}, character::complete::{multispace0, alpha1, digit1, alphanumeric0, alphanumeric1}, multi::separated_list1};

fn main() {
    println!("Hello, world!");

    let input: &str = "9eightone
    hczsqfour3nxm5seven4
    9twopjqkghmbone
    rhrfthv886vflthreeztvzs
    tlbtwo62five
    ninetwonine234nvtlzxzczx";

    // let total = day_one_two(input);

    // day_two_one(12, 13, 14);
    // day_two_two();
    // day_three_one();
    // day_three_two();
    // day_four_one();
    // day_four_two();
    // day_five_one();
    // day_six_one();
    // day_six_two();
    // day_seven_one();
    // day_eight_one();
    // day_eight_two();
    // day_nine_one();
    // day_nine_two();
    // day_ten_one();
    day_ten_two();
    //println!("{}", total);
}

fn day_one_two(input: &str) -> i32 {

    let lines = input.split("\n");
    let mut total: i32 = 0;

    for line in lines {
        let mut results: Vec<char> = vec![];
        let mut i: i32 = 0;

        while i < line.len() as i32{
            find_word(line.chars().collect(), &mut i , &mut results);
            i += 1;
        }

        if results.len() > 0 {
            let n: String = vec![results[0], results[results.len() - 1]].iter().collect();
            println!("{} {:?}", n, results);
            total += n.parse::<i32>().unwrap();
        }
    }

    return total;
}

fn day_one_one(input: &str) -> i32 {
    let lines = input.split("\n");
    let mut total = 0;

    for line in lines {
        let line_val: Vec<char> = line.chars().filter(|c: &char| c > &'0' && c <= &'9').collect();
        if line_val.len() > 0 {
            let char_str: String = vec![line_val[0], line_val[line_val.len() - 1]].iter().collect();
            total += char_str.parse::<i32>().unwrap();
        }
    }

    return total;
}

fn find_word(line: Vec<char>, index: &mut i32, results: &mut Vec<char>) {
    let mut i = *index as usize;

    if i < line.len() {
        let ci = line[i];

        if line[i] > '0' && line[i] <= '9' {
            results.push(line[i]);
            *index = i as i32;
            return;
        }

        if line[i] == 'o' {
            if i+1 < line.len() && line[i+1] == 'n' {
                if i+2 < line.len() && line[i+2] == 'e' {
                    results.push('1');
                    *index = i as i32;
                    return;
                }
            }
        }

        if line[i] == 't' {
            if i+1 < line.len() && line[i+1] == 'w' {
                if i+2 < line.len() && line[i+2] == 'o' {
                    results.push('2');
                    *index = i as i32;
                    return;
                }
            }

            if i+1 < line.len() && line[i+1] == 'h' {
                if i+2 < line.len() && line[i+2] == 'r' {
                    if i+3 < line.len() && line[i+3] == 'e' {
                        if i+4 < line.len() && line[i+4] == 'e' {
                            results.push('3');
                            *index = i as i32;
                            return;
                        }
                    }
                }
            }
        }

        if i < line.len() && line[i] == 'f' {
            if i+1 < line.len() && line[i+1] == 'o' {
                if i+2 < line.len() && line[i+2] == 'u' {
                    if i+3 < line.len() && line[i+3] == 'r' {
                        results.push('4');
                        *index = i as i32;
                        return;
                    }
                }
            }

            if i+1 < line.len() && line[i+1] == 'i' {
                if i+2 < line.len() && line[i+2] == 'v' {
                    if i+3 < line.len() && line[i+3] == 'e' {
                        results.push('5');
                        *index = i as i32;
                        return;
                    }
                }
            }
        }

        if i < line.len() && line[i] == 's' {
            if i+1 < line.len() && line[i+1] == 'i' {
                if i+2 < line.len() && line[i+2] == 'x' {
                    results.push('6');
                    *index = i as i32;
                    return;
                }
            }

            if i+1 < line.len() && line[i+1] == 'e' {
                if i+2 < line.len() && line[i+2] == 'v' {
                    if i+3 < line.len() && line[i+3] == 'e' {
                        if i+4 < line.len() && line[i+4] == 'n' {
                            results.push('7');
                            *index = i as i32;
                            return;
                        }
                    }
                }
            }
        }

        if i < line.len() && line[i] == 'e' {
            if i+1 < line.len() && line[i+1] == 'i' {
                if i+2 < line.len() && line[i+2] == 'g' {
                    if i+3 < line.len() && line[i+3] == 'h' {
                        if i+4 < line.len() && line[i+4] == 't' {
                            results.push('8');
                            *index = i as i32;
                            return;
                        }
                    }
                }
            }
        }

        if i < line.len() && line[i] == 'n' {
            if i+1 < line.len() && line[i+1] == 'i' {
                if i+2 < line.len() && line[i+2] == 'n' {
                    if i+3 < line.len() && line[i+3] == 'e' {
                        results.push('9');
                        *index = i as i32;
                        return;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Game {
    id: String,
    guesses: Vec<Guess>
}

#[derive(Debug)]
struct Guess {
    pub count: i32,
    pub color: String
}

fn is_digit(c: char) -> bool {
    return c.is_digit(10);
}

fn is_space(c: char) -> bool {
    return c.is_whitespace();
}

fn parse_color(input: &str) -> IResult<&str, &str>{
    return preceded(multispace0, alpha1)(input);
}

fn parse_count(input: &str) -> IResult<&str, &str>{
    return preceded(multispace0, digit1)(input);
}

fn parse_guess(input: &str) -> IResult<&str, Guess> {

    let (input, count) = parse_count(input)?;
    let (input, _) = multispace0(input)?;
    let (input, color) = parse_color(input)?;

    return Ok((input, Guess { color: color.to_string(), count: count.parse::<i32>().unwrap()}));
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = multispace0(input)?;
    let (input, id) = delimited(tag_no_case("Game "), digit1, tag(":"))(input)?;
    let (input, moves) = separated_list1(alt((tag(";"), tag(","))), parse_guess)(input)?;

    return Ok((input, Game { id: id.to_string(), guesses: moves}));
}

fn day_two_one(red: i32, green: i32, blue: i32) -> i32 {

    let input1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";

    let input = "Game 1: 2 red, 2 green; 1 red, 1 green, 2 blue; 3 blue, 3 red, 3 green; 1 blue, 3 green, 7 red; 5 red, 3 green, 1 blue
    Game 2: 5 green, 4 red, 7 blue; 7 red, 4 green, 4 blue; 8 green, 11 blue, 4 red; 2 red, 18 blue, 3 green; 7 red, 15 blue
    Game 3: 2 green, 4 blue; 2 red, 2 green; 6 red, 1 green; 2 red, 1 green; 2 green; 5 blue, 5 red
    Game 4: 10 red, 7 green, 10 blue; 8 red, 2 green; 9 green, 6 red, 5 blue; 8 green, 2 blue, 4 red; 5 green, 9 blue; 10 red, 1 green, 9 blue
    Game 5: 10 blue, 7 green, 2 red; 2 blue, 4 red; 2 green, 9 blue, 8 red
    Game 6: 3 green, 8 red; 1 blue, 11 red, 2 green; 2 green, 15 red, 8 blue; 13 red, 6 blue, 3 green
    Game 7: 4 green, 10 red, 7 blue; 6 red, 9 blue, 9 green; 2 red, 1 blue, 6 green";

    let lines = input.split("\n");

    let mut all_maps: Vec<(i32, HashMap<String, i32>)> = vec![];

    for line in lines {
        match parse_game(line) {
            Ok((_, game)) => {
                let res: Vec<(String, i32)> = game.guesses.iter().map(|game| (game.color.clone(), game.count)).collect();
                let mut res_map: HashMap<String, i32> = HashMap::new();

                for r in res {
                    match res_map.get(&r.0) {
                        Some(v) => {
                            res_map.insert(r.0, r.1.max(*v));
                        }
                        None => {
                            res_map.insert(r.0, r.1);
                        }
                    }
                }

                all_maps.push((game.id.trim().parse::<i32>().unwrap(), res_map));
            }

            Err(error) => {
                println!("{}", error);
            }
        }
    }

    let mut total = 0;

    for m in all_maps.clone() {
        println!("{:?} {}", m, all_maps.len());

        match m.1.get("red") {
            None => {
                if red == 0 {
                    continue;
                }
            }
            Some(v) => {
                println!("sent: {} bag: {}", red, v);
                if red < *v {
                    continue;
                }
            }
        }

        match m.1.get("blue") {
            None => {
                if blue == 0 {
                    continue;
                }
            }
            Some(v) => {
                println!("sent: {} bag: {}", blue, v);
                if blue < *v {
                    continue;
                }
            }
        }

        match m.1.get("green") {
            None => {
                if green == 0 {
                    continue;
                }
            }
            Some(v) => {
                println!("sent: {} bag: {}", green, v);
                if green < *v {
                    continue;
                }
            }
        }

        total += m.0;

    }

    println!("{:?}", all_maps);
    println!("{}", total);
    return total;
}


fn day_two_two() -> i32 {

    let input = "Game 1: 2 red, 2 green; 1 red, 1 green, 2 blue; 3 blue, 3 red, 3 green; 1 blue, 3 green, 7 red; 5 red, 3 green, 1 blue
    Game 2: 5 green, 4 red, 7 blue; 7 red, 4 green, 4 blue; 8 green, 11 blue, 4 red; 2 red, 18 blue, 3 green; 7 red, 15 blue
    Game 3: 2 green, 4 blue; 2 red, 2 green; 6 red, 1 green; 2 red, 1 green; 2 green; 5 blue, 5 red
    Game 4: 10 red, 7 green, 10 blue; 8 red, 2 green; 9 green, 6 red, 5 blue; 8 green, 2 blue, 4 red; 5 green, 9 blue; 10 red, 1 green, 9 blue
    Game 5: 10 blue, 7 green, 2 red; 2 blue, 4 red; 2 green, 9 blue, 8 red
    Game 6: 3 green, 8 red; 1 blue, 11 red, 2 green; 2 green, 15 red, 8 blue; 13 red, 6 blue, 3 green
    Game 7: 4 green, 10 red, 7 blue; 6 red, 9 blue, 9 green; 2 red, 1 blue, 6 green
    Game 8: 1 red, 3 blue, 2 green; 7 green, 2 blue; 10 green, 1 red, 2 blue; 1 red
    Game 9: 4 red, 3 green, 11 blue; 6 red, 4 green; 15 red, 7 blue, 7 green
    Game 10: 7 red, 1 blue, 5 green; 11 red, 7 green, 1 blue; 2 green, 4 blue, 13 red";

    let lines = input.split("\n");

    let mut all_maps: Vec<(i32, HashMap<String, i32>)> = vec![];

    for line in lines {
        match parse_game(line) {
            Ok((_, game)) => {
                let res: Vec<(String, i32)> = game.guesses.iter().map(|game| (game.color.clone(), game.count)).collect();
                let mut res_map: HashMap<String, i32> = HashMap::new();

                for r in res {
                    match res_map.get(&r.0) {
                        Some(v) => {
                            res_map.insert(r.0, r.1.max(*v));
                        }
                        None => {
                            res_map.insert(r.0, r.1);
                        }
                    }
                }

                all_maps.push((game.id.trim().parse::<i32>().unwrap(), res_map));
            }

            Err(error) => {
                println!("{}", error);
            }
        }
    }

    let mut total = 0;

    for m in all_maps.clone() {
        println!("{:?} {}", m, all_maps.len());

        total += m.1.get("red").unwrap_or(&0) * m.1.get("green").unwrap_or(&0) * m.1.get("blue").unwrap_or(&0);
    }

    println!("{}", total);
    return total;
}

fn check_if_symbol(c: char) -> (bool, bool) {
    if c == '*' {
        return (true, true);
    }

    return (!c.is_digit(10) && c != '.', false);
}

fn check_adjacent(m: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    
    let height = m.len();
    let width = m[0].len();
    let mut adjacent = false;
    let mut is_gear = false;
    let mut gear_location = (0,0);
    if y > 0 && x > 0 {
        if check_if_symbol(m[y - 1][x - 1]).0 {
//            println!("top left");
            adjacent = true;
        }
    }

    if y < height - 1 && x < width - 1 {
        if check_if_symbol(m[y + 1][x + 1]).0 {
//            println!("bottom right {} {} {} {}", y, x, m[y][x], m[y+1][x+1]);
            adjacent = true;
        }
    }

    if y > 0 && x < width - 1 {
        if check_if_symbol(m[y - 1][x + 1]).0 {
//            println!("top right");
            adjacent = true;
        }
    }

    if y < height - 1 && x > 0 {
        if check_if_symbol(m[y + 1][x - 1]).0 {
//            println!("bottom left");
            adjacent = true;
        }
    }

    if y > 0 {
        if check_if_symbol(m[y - 1][x]).0 {
//            println!("top");
            adjacent = true;
        }
    }

    if y < height - 1 {
        if check_if_symbol(m[y + 1][x]).0 {
//           println!("bottom");
            adjacent = true;
        }
    }

    if x > 0 {
        if check_if_symbol(m[y][x - 1]).0 {
//            println!("left");
            adjacent = true;
        }
    }

    if x < width - 1 {
        if check_if_symbol(m[y][x + 1]).0 {
//            println!("right");
            adjacent = true;
        }
    }

    return adjacent;
}

fn check_adjacent_gears(m: &Vec<Vec<char>>, x: usize, y: usize) -> HashSet<(usize, usize)> {
    
    let height = m.len();
    let width = m[0].len();
    let mut gears: HashSet<(usize, usize)> = HashSet::new();

    if y > 0 && x > 0 {
        let symbol = check_if_symbol(m[y - 1][x - 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y - 1, x - 1));
            }
        }
    }

    if y < height - 1 && x < width - 1 {

        let symbol = check_if_symbol(m[y + 1][x + 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y + 1, x + 1));
            }
        }
    }

    if y > 0 && x < width - 1 {
        let symbol = check_if_symbol(m[y - 1][x + 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y - 1, x + 1));
            }
        }
    }

    if y < height - 1 && x > 0 {
        let symbol = check_if_symbol(m[y + 1][x - 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y + 1, x - 1));
            }
        }
    }

    if y > 0 {
        let symbol = check_if_symbol(m[y - 1][x]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y - 1, x));
            }
        }
    }

    if y < height - 1 {
        let symbol = check_if_symbol(m[y + 1][x]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y + 1, x));
            }
        }
    }

    if x > 0 {
        let symbol = check_if_symbol(m[y][x - 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y, x - 1));
            }
        }
    }

    if x < width - 1 {
        let symbol = check_if_symbol(m[y][x + 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y, x + 1));
            }
        }
    }

    return gears;
}

fn day_three_two() {

    let input2 = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    let input: &str = 
    "...................305.124................................432..............................................576..313.....514.................
    .............113...-......&....................&...819...........654..../..........................&901................*....869.257.........
    ...377..&783../.................................9...........855*......940..463................-.........................844.*....@......679.
    ......*...........197.261.....817..336.759............&742......548.......&........748......844.............#.......&........254...169..*...
    .......36....368.*...............*....*.........*..88......%866.......135.........*..................515.682.....114...%...........*.....768";

    let mut l: Vec<&str> = input.split("\n").collect();
    let ll: Vec<Vec<char>> = l.iter_mut().map(|li| li.trim().chars().collect()).collect();

    let mut number_flag: String = String::new();
    let mut is_adjacent_flag: bool = false;
    let mut gears: HashSet<(usize, usize)> = HashSet::new();
    let mut all_gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    let mut total = 0;
    for (j, row) in ll.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            if *v == '.' || i == 0 || check_if_symbol(*v).0{
                if !number_flag.is_empty() {
                    if is_adjacent_flag == true {
                        println!("{}", number_flag.parse::<i32>().unwrap());
                        let n = number_flag.parse::<i32>().unwrap();

                        for gear in gears {
                            match all_gears.get(&gear) {
                                None => {
                                    all_gears.insert(gear, vec![n]);
                                }
                                Some(g) => {
                                    let mut h = vec![n];
                                    h.extend(g);
                                    all_gears.insert(gear, h);
                                }
                            }
                        }
                    }
                }

                number_flag = String::new();
                is_adjacent_flag = false;
                gears = HashSet::new();
            }

            if v.is_digit(10) {
                number_flag.push(*v);
                
                if is_adjacent_flag == false {
                    is_adjacent_flag = check_adjacent(&ll, i, j);
                    gears.extend(check_adjacent_gears(&ll, i, j));
                }
            }
        }
    }

    println!("{:?}", all_gears);
    for elem in all_gears {
        if elem.1.len() == 2 {
            total += elem.1[0] * elem.1[1];
        }
    }
    println!("{}", total);
}

fn day_three_one() {
    let input = "12.......*..
    +.........34
    .......-12..
    ..78........
    ..*....60...
    78..........
    .......23...
    ....90*12...
    ............
    2.2......12.
    .*.........*
    1.1.......56";

    let mut l: Vec<&str> = input.split("\n").collect();
    let ll: Vec<Vec<char>> = l.iter_mut().map(|li| li.trim().chars().collect()).collect();

    let mut number_flag: String = String::new();
    let mut is_adjacent_flag: bool = false;
    let mut gear_map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    let mut total = 0;
    for (j, row) in ll.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            if *v == '.' || i == 0 || check_if_symbol(*v).0{
                if !number_flag.is_empty() {
                    if is_adjacent_flag == true {
                        println!("{}", number_flag.parse::<i32>().unwrap());
                        total += number_flag.parse::<i32>().unwrap();
                    }
                }

                number_flag = String::new();
                is_adjacent_flag = false;
            }

            if v.is_digit(10) {
                number_flag.push(*v);
                
                if is_adjacent_flag == false {
                    is_adjacent_flag = check_adjacent(&ll, i, j);
                }
            }
        }
    }
    println!("{}", total);
}

fn day_four_one() {
    let input = "Card   1: 30 48 49 69  1 86 94 68 12 85 | 86 57 89  8 81 85 82 68  1 22 90  2 74 12 30 45 69 92 62  4 94 48 47 64 49
    Card   2: 57 32 92 73 76 62 11 19 61 90 | 19 82 53 87 57 80 69 76 90 56 11 61 30 92 73 99  4 32 33 64 34 62 27 78 65
    Card   3:  6 56 40  1 47 26 25 87  4  2 | 12 26 32 25  8  4 41 54 69 99  2 45 70  6 59 23 47  7 49 17  1 56 92 87 40
    Card   4: 26 49 44 82 25 43 47 74 97 13 | 76 62 13 82 55 26 93 84 83 19 47 22 49 44 25 43  7 18  9 45 97 15 90 85 74
    Card   5: 88 58 96 80 56 16 55 13  3 40 | 20 57 23 71 76 43 36 72 52 18 60 28 80 84 64 75 93 46 19 69 25 31 58 45 47
    Card   6: 89 88 34 62 60 41 15 42 57 58 | 58 49 82 42 70 78 72 57 77 47 62 89 30 60 96 98 54 66 25 14  6 34 15 41 88";

    let lines = input.split("\n");

    let mut total = 0;

    for line in lines {
        let h: Vec<&str> = line.split(":").collect();
        let h2 = h[1].replace("  ", " ");
        let numbers: Vec<&str> = h2.trim().split("|").collect();
        let winning_numbers_str: Vec<&str> = numbers[0].trim().split(" ").collect();
        let winning_numbers: HashSet<i32> = HashSet::from_iter(winning_numbers_str.iter().map(| n | n.trim().parse::<i32>().unwrap()));
        let numbers_str: Vec<&str> = numbers[1].trim().split(" ").collect();
        let numbers = numbers_str.iter().map(| n | n.trim().parse::<i32>().unwrap());

        let mut inner_total = 0;
        for number in numbers {
            if winning_numbers.contains(&number) {
                if inner_total == 0 {
                    inner_total = 1;
                } else {
                    inner_total = inner_total * 2;
                }
            }
        }

        total += inner_total;
    }

    println!("{}", total);
}

fn day_four_two() {

    let input = "Card   1: 30 48 49 69  1 86 94 68 12 85 | 86 57 89  8 81 85 82 68  1 22 90  2 74 12 30 45 69 92 62  4 94 48 47 64 49
    Card   2: 57 32 92 73 76 62 11 19 61 90 | 19 82 53 87 57 80 69 76 90 56 11 61 30 92 73 99  4 32 33 64 34 62 27 78 65
    Card   3:  6 56 40  1 47 26 25 87  4  2 | 12 26 32 25  8  4 41 54 69 99  2 45 70  6 59 23 47  7 49 17  1 56 92 87 40
    Card   4: 26 49 44 82 25 43 47 74 97 13 | 76 62 13 82 55 26 93 84 83 19 47 22 49 44 25 43  7 18  9 45 97 15 90 85 74
    Card   5: 88 58 96 80 56 16 55 13  3 40 | 20 57 23 71 76 43 36 72 52 18 60 28 80 84 64 75 93 46 19 69 25 31 58 45 47
    Card   6: 89 88 34 62 60 41 15 42 57 58 | 58 49 82 42 70 78 72 57 77 47 62 89 30 60 96 98 54 66 25 14  6 34 15 41 88";
    
    let lines: Vec<&str> = input.split("\n").collect();
    let mut copies_map: HashMap<i32, i32> = HashMap::new();
    let mut total = 0;

    for i in 0..lines.len() {
        let h: Vec<&str> = lines[i].split(":").collect();
        let h2 = h[1].replace("  ", " ");
        let n2: Vec<&str> = h2.trim().split("|").collect();
        let winning_numbers_str: Vec<&str> = n2[0].trim().split(" ").collect();
        let winning_numbers: HashSet<i32> = HashSet::from_iter(winning_numbers_str.iter().map(| n | n.trim().parse::<i32>().unwrap()));
        let numbers_str: Vec<&str> = n2[1].trim().split(" ").collect();
        let numbers = numbers_str.iter().map(| n | n.trim().parse::<i32>().unwrap());

        let mut matching_numbers = 0;
        for number in numbers {
            if winning_numbers.contains(&number) {
                matching_numbers += 1;
            }
        }
        
        match copies_map.get(&(i as i32)) {
            None => {
                copies_map.insert(i as i32, 1);
            }
            Some(v) => {
                copies_map.insert(i as i32, v + 1);
            }
        }

        for j in 0..matching_numbers {
            match copies_map.get(&((i as i32) + 1 + (j as i32))) {
                None => {
                    copies_map.insert((i as i32) + 1 + (j as i32), *copies_map.get(&(i as i32)).unwrap());
                }
                Some(v) => {
                    copies_map.insert((i as i32) + 1 + (j as i32), v + *copies_map.get(&(i as i32)).unwrap());
                }
            }
        }
    }

    let all_values: Vec<&i32> =  copies_map.values().collect();

    let sum: i32 = all_values.iter().copied().sum();
    
    println!("Sum: {}", sum);
}

fn day_five_one() {
    let input = "seeds: 2276375722 160148132 3424292843 82110297 1692203766 342813967 3289792522 103516087 2590548294 590357761 1365412380 80084180 3574751516 584781136 4207087048 36194356 1515742281 174009980 6434225 291842774

    seed-to-soil map:
    4170452318 3837406401 124514978
    2212408060 1593776674 105988696
    3837406401 4016132523 278834773
    1475766470 1699765370 492158296
    3698488336 1475766470 118010204";

    let mut seed_to_soil: Vec<(i64, i64, i64)> = vec![];
    let mut soil_to_fertilizer: Vec<(i64, i64, i64)> = vec![];
    let mut fertilizer_to_water: Vec<(i64, i64, i64)> = vec![];
    let mut water_to_light: Vec<(i64, i64, i64)> = vec![];
    let mut light_to_temperature: Vec<(i64, i64, i64)> = vec![];
    let mut temperature_to_humidity: Vec<(i64, i64, i64)> = vec![];
    let mut humidity_to_location: Vec<(i64, i64, i64)> = vec![];
    let lines: Vec<&str> = input.split("\n").collect();

    let split_seeds: Vec<&str> = lines[0].trim().split(":").collect();
    let seeds: Vec<i64> = split_seeds[1].replace("  ", " ").trim().split(" ").map(|s| s.parse::<i64>().unwrap()).collect();

    let mut index = 1;
    let lines_length = lines.len();

    while index < lines_length {
        let line = &lines[index];

        if (*line).is_empty() {
            index += 1;
            continue;
        } else {
            if line.trim() == "seed-to-soil map:" {
                get_from_lines(&lines, &mut index, &mut seed_to_soil);
            }

            if line.trim() == "soil-to-fertilizer map:" {
                get_from_lines(&lines, &mut index, &mut soil_to_fertilizer);
            }

            if line.trim() == "fertilizer-to-water map:" {
                get_from_lines(&lines, &mut index, &mut fertilizer_to_water);
            }

            if line.trim() == "water-to-light map:" {
                get_from_lines(&lines, &mut index, &mut water_to_light);
            }

            if line.trim() == "light-to-temperature map:" {
                get_from_lines(&lines, &mut index, &mut light_to_temperature);
            }

            if line.trim() == "temperature-to-humidity map:" {
                get_from_lines(&lines, &mut index, &mut temperature_to_humidity);
            }

            if line.trim() == "humidity-to-location map:" {
                get_from_lines(&lines, &mut index, &mut humidity_to_location);
            }

            index += 1;
        }
    }

    let mut res: Vec<i64> = vec![];
    let mut i = 0;
    while i < seeds.len() {
        for seed in seeds[i]..seeds[i] + seeds[i+1] {
            let mut final_seed = seed as i64;
            for seed_to_soil_range in &seed_to_soil {
                
                let range_down = seed_to_soil_range.1;
                let range_up = range_down + seed_to_soil_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (seed_to_soil_range.0 - seed_to_soil_range.1);
 //                   println!("seed-to-soil {:?} = range-up: {}, range-down: {}, adding: {}", seed_to_soil, range_up, range_down,(seed_to_soil_range.0 - seed_to_soil_range.1));
                    break;
                }
            }

            for soil_to_fertilizer_range in &soil_to_fertilizer {
                let range_down = soil_to_fertilizer_range.1;
                let range_up = range_down + soil_to_fertilizer_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (soil_to_fertilizer_range.0 - soil_to_fertilizer_range.1);
 //                   println!("soil-to-fertilizer {:?} = range-up: {}, range-down: {}, adding: {}", soil_to_fertilizer, range_up, range_down,(soil_to_fertilizer_range.0 - soil_to_fertilizer_range.1));
                    break;
                }
            }

            for fertilizer_to_water_range in &fertilizer_to_water {
                let range_down = fertilizer_to_water_range.1;
                let range_up = range_down + fertilizer_to_water_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (fertilizer_to_water_range.0 - fertilizer_to_water_range.1);
 //                   println!("fertilizer-to-water {:?} = range-up: {}, range-down: {}, adding: {}", fertilizer_to_water, range_up, range_down,(fertilizer_to_water_range.0 - fertilizer_to_water_range.1));
                    break;
                }
            }

            for water_to_light_range in &water_to_light {
                let range_down = water_to_light_range.1;
                let range_up = range_down + water_to_light_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (water_to_light_range.0 - water_to_light_range.1);
 //                   println!("water_to_light {:?} = range-up: {}, range-down: {}, adding: {}", water_to_light, range_up, range_down,(water_to_light_range.0 - water_to_light_range.1));
                    break;
                }
            }

            for light_to_temperature_range in &light_to_temperature {
                let range_down = light_to_temperature_range.1;
                let range_up = range_down + light_to_temperature_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (light_to_temperature_range.0 - light_to_temperature_range.1);
//                    println!("light_to_temperature {:?} = range-up: {}, range-down: {}, adding: {}", light_to_temperature, range_up, range_down,(light_to_temperature_range.0 - light_to_temperature_range.1));
                    break;
                }
            }

            for temperature_to_humidity_range in &temperature_to_humidity {
                let range_down = temperature_to_humidity_range.1;
                let range_up = range_down + temperature_to_humidity_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (temperature_to_humidity_range.0 - temperature_to_humidity_range.1);
//                    println!("temperature_to_humidity {:?} = range-up: {}, range-down: {}, adding: {}", temperature_to_humidity, range_up, range_down,(temperature_to_humidity_range.0 - temperature_to_humidity_range.1));
                    break;
                }
            }

            for humidity_to_location_range in &humidity_to_location {
                let range_down = humidity_to_location_range.1;
                let range_up = range_down + humidity_to_location_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (humidity_to_location_range.0 - humidity_to_location_range.1);
//                    println!("humidity_to_location {:?} = range-up: {}, range-down: {}, adding: {}", humidity_to_location, range_up, range_down,(humidity_to_location_range.0 - humidity_to_location_range.1));
                    break;
                }
            }

            res.push(final_seed);
            // println!("final seed: {}", final_seed);
        }

        i += 2;
    }

    println!("minimum: {:?}", res.iter().min());
}

fn get_from_lines(lines: &Vec<&str>, index: &mut usize, vec: &mut Vec<(i64, i64, i64)>){
    *index += 1;
    while *index < lines.len() && !lines[*index].trim().is_empty() {
        let v: Vec<i64> = lines[*index].trim().replace("  ", " ").split(" ").map(| s | s.parse::<i64>().unwrap()).collect();
        vec.push((v[0], v[1], v[2]));
        *index += 1;
    }
}

fn day_six_one() {

    let input = "Time:      7  15   30
    Distance:  9  40  200";

    let lines: Vec<&str> = input.split("\n").collect();

    // Times
    let times_line: Vec<&str> = lines[0].split(":").collect();
    let times_str_trim: String = times_line[1].trim().to_string();
    let times_str: Vec<&str> = times_str_trim.split_whitespace().collect();
    let times: Vec<i32> = times_str.iter().map(| t | t.parse::<i32>().unwrap()).collect();

    // Distance
    let distances_line: Vec<&str> = lines[1].split(":").collect();
    let distances_str_trim: String = distances_line[1].trim().to_string();
    let distances_str: Vec<&str> = distances_str_trim.split_whitespace().collect();
    let distances: Vec<i32> = distances_str.iter().map(| t | t.parse::<i32>().unwrap()).collect();

    for i in 0..times.len() {
        let mut inner_total = 0;
        for ii in 0..times[i] as usize {
            // println!("time: {} distance: {} time hold: {} difference: {} result: {}", times[i], distances[i], ii, times[i] as usize - ii, ii * (times[i] as usize - ii));
            if ii * (times[i] as usize - ii) > distances[i] as usize {
                inner_total += 1;
            }
        }
        
        println!("{}", inner_total);
    }
}


fn day_six_two() {

    let input = "Time:        49     87     78     95
    Distance:   356   1378   1502   1882";

    let lines: Vec<&str> = input.split("\n").collect();

    // Times
    let times_line: Vec<&str> = lines[0].split(":").collect();
    let times_str_trim: String = times_line[1].trim().to_string().replace(" ", "");
    let time: i64 = (&times_str_trim[..]).trim().parse::<i64>().unwrap();

    // Distance
    let distances_line: Vec<&str> = lines[1].split(":").collect();
    let distances_str_trim: String = distances_line[1].trim().to_string().replace(" ", "");
    let distance: i64 = (&distances_str_trim[..].trim()).parse::<i64>().unwrap();

    let mut total = 0;

    let mut flag = false;
    for i in 0..time as usize {
        if i * (time as usize - i) >= distance as usize {
            total += 1;
            if !flag {
                flag = true;
                println!("time hold {} time diff {} time {} distance {}", i, i * (time as usize - i), time, distance);
            }
        }
    }
    
    println!("{}", total);
}

fn to_tuple(s: &str) -> (&str, i32) {
    let split: Vec<&str> = s.trim().split(" ").collect();
    return (&split[0].trim(), split[1].trim().parse::<i32>().unwrap());
}

fn get_hand_rank_vec(hand: Vec<&i32>, j: i32) -> i32 {


    let mut sorted_hand = hand.clone();
    sorted_hand.sort_by(|x, y | if &&y < &&x { Ordering::Less } else { Ordering::Greater });

//    println!("sorted hand {:?}", sorted_hand);

    if (*sorted_hand).is_empty() {
        return 7;
    }
    
    if *sorted_hand[0] + j == 5 {
        return 7;    
    }

    if *sorted_hand[0] + j == 4 {
        return 6;
    }

    if *sorted_hand[0] + j == 3 {

        if (*sorted_hand).len() > 1 && *sorted_hand[1] == 2 {
            return 5;
        }

        if (*sorted_hand).len() > 1 && *sorted_hand[1] == 1 {
            return 4;
        }
    }

    if *sorted_hand[0] + j == 2 {
        if (*sorted_hand).len() > 1 && *sorted_hand[1] == 2 {
            return 3;
        }

        if (*sorted_hand).len() > 1 && *sorted_hand[1] == 1 {
            return 2;
        }
    }

    return 1;
}

fn get_hand_rank(hand: &str) -> i32 {
    let mut m: HashMap<char, i32> = HashMap::new();

    let mut j = 0;
    for c in hand.chars() {
        match m.get(&c) {
            None => {
                if c == 'J' {
                    j += 1;
                } else {
                    m.insert(c, 1);
                }
            }
            Some(v) => {
                m.insert(c, v+1);
            }
        }
    }

    let values: Vec<&i32> = m.values().collect();
    let v = values.clone();
//  println!("hand: {} map: {:?} rank: {}", hand, m, get_hand_rank_vec(v));
    return get_hand_rank_vec(values, j);
}

fn compare_to_sort(hand1: &(&str, i32), hand2: &(&str, i32)) -> Ordering {

    if get_hand_rank(hand1.0) > get_hand_rank(hand2.0) {
        return Ordering::Greater;
    } 

    if get_hand_rank(hand1.0) < get_hand_rank(hand2.0) {
        return Ordering::Less;
    } 

    if get_hand_rank(hand1.0) == get_hand_rank(hand2.0) {
        for i in 0..5 {
            let card1 = hand1.0.chars().collect::<Vec<char>>()[i];
            let card2 = hand2.0.chars().collect::<Vec<char>>()[i];

            let card1_val: i32 = if card1 == 'A' {
                14
            } else if card1 == 'K' {
                13
            } else if card1 == 'Q' {
                12
            } else if card1 == 'J' {
                0
            } else if card1 == 'T' {
                10
            }else {
                //println!("{}", card1);
                card1.to_digit(10).unwrap() as i32
            };

            let card2_val: i32 = if card2 == 'A' {
                14
            } else if card2 == 'K' {
                13
            } else if card2 == 'Q' {
                12
            } else if card2 == 'J' {
                0
            } else if card2 == 'T' {
                10
            } else {
                card2.to_digit(10).unwrap() as i32
            };

            if card1_val > card2_val {
                return Ordering::Greater;
            }

            if card1_val < card2_val {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }

    return Ordering::Equal;
}

fn day_seven_one() {

    let input = "K99QT 53
    TKQ7T 320
    22A7J 490
    267J9 69
    665JJ 431
    K856J 605";

    let lines_str: Vec<&str> = input.split("\n").collect();
    let mut lines: Vec<(&str, i32)> = lines_str.iter().map(|s| to_tuple(&s)).collect();

    lines.sort_by(compare_to_sort);

    let mut total = 0;

//  println!("{:?}", lines);
    for i in 0..lines.len() {
        total += lines[i].1 * (i as i32 + 1);
//        println!("rank: {}, bid: {}, bid * rank: {}, total: {}", i + 1, lines[i].1, lines[i].1 * (i as i32 + 1), total);
    }

    println!("{}", total);
}

fn parse_map_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, key) = preceded(multispace0, alpha1)(input)?;
    let (input, _) = delimited(multispace0, tag("="), multispace0)(input)?;
    let (input, map_els) = delimited(tag("("), separated_list1(tag(","), preceded(multispace0, alpha1)), tag(")"))(input)?;

    return Ok((input, (key, (map_els[0], map_els[1]))));
}

fn parse_map_line_2(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, key) = preceded(multispace0, alphanumeric1)(input)?;
    let (input, _) = delimited(multispace0, tag("="), multispace0)(input)?;
    let (input, map_els) = delimited(tag("("), separated_list1(tag(","), preceded(multispace0, alphanumeric1)), tag(")"))(input)?;

    return Ok((input, (key, (map_els[0], map_els[1]))));
}

fn day_eight_one() {
    let input = "LRLRLRRLRRRLRRRLRRRLRLRRRLRRRLRRRLLRLRRRLRLRRRLLRRRLRRLRRRLRRLRLRRRLRRRLRLRRLRRRLRRLRRRLRRLRLRRLRRRLRLRRLRRRLLRRRLRLRRLLLRLLRLRRLLRRRLLRLLRRLRLRRRLLLRLRRLRLRRLRRRLRRLLRRLLRLRRRLRRRLRLLLLRLLRLRLRLRRRLRRLRRLRLRRRLLRRLRLLRRLRLRRLRLRLRRLRRLLRLRRLLRLLRRRLLLRRRLRRLRLRRRLRRLRRRLRRLLLRRRR

    QKX = (SQD, XTJ)
    FKP = (JGJ, JPR)
    VCQ = (XGB, SSS)
    JSK = (LSQ, FFS)
    LGF = (THC, RQQ)
    HSQ = (TBN, XTK)";

    let lines = input.lines().collect::<Vec<&str>>();

    let instructions: Vec<char> = lines[0].trim().chars().collect::<Vec<char>>();
    let mut m: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current = "AAA";
    let last = "ZZZ";
    for i in 2..lines.len() {
        let input = lines[i];
        match parse_map_line(input) {
            Err(err) => {
                println!("{}", err)
            }
            Ok(res) => {
                m.insert(res.1.0, res.1.1);
            }
        }
    }

    let mut instruction_curr = 0;
    let mut total: i128 = 1;

    println!("lqwkjerlkr {}", last);

    while current != last {
        if instruction_curr >= instructions.len() {
            instruction_curr = 0;
        }

        if instructions[instruction_curr] == 'R' {
            let c = m.get(current).unwrap();
            current = c.1;
        }

        if instructions[instruction_curr] == 'L' {
            let c = m.get(current).unwrap();
            current = c.0;
        }

        total += 1;
        instruction_curr += 1;    
    }
    println!("total {}", total);
}

fn all_current_on_z(vec: &Vec<&str>) -> bool {
    return vec.iter().all(|v| v.chars().collect::<Vec<char>>()[2] == 'Z');
}

fn day_eight_two() {
    let input = "LRLRLRRLRRRLRRRLRRRLRLRRRLRRRLRRRLLRLRRRLRLRRRLLRRRLRRLRRRLRRLRLRRRLRRRLRLRRLRRRLRRLRRRLRRLRLRRLRRRLRLRRLRRRLLRRRLRLRRLLLRLLRLRRLLRRRLLRLLRRLRLRRRLLLRLRRLRLRRLRRRLRRLLRRLLRLRRRLRRRLRLLLLRLLRLRLRLRRRLRRLRRLRLRRRLLRRLRLLRRLRLRRLRLRLRRLRRLLRLRRLLRLLRRRLLLRRRLRRLRLRRRLRRLRRRLRRLLLRRRR

    QKX = (SQD, XTJ)
    FKP = (JGJ, JPR)
    VCQ = (XGB, SSS)
    JSK = (LSQ, FFS)";

    let lines = input.lines().collect::<Vec<&str>>();

    let instructions: Vec<char> = lines[0].trim().chars().collect::<Vec<char>>();
    let mut m: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current_nodes: Vec<&str> = vec![];

    for i in 2..lines.len() {
        let input = lines[i];
        match parse_map_line_2(input) {
            Err(err) => {
                println!("{}", err)
            }
            Ok(res) => {
                m.insert(res.1.0, res.1.1);
            }
        }
    }

    for key in m.keys() {
        let char_key: Vec<char> = (*key).chars().collect();

        if char_key[2] == 'A' {
            current_nodes.push(*key);
        }
    }

    let mut instruction_curr = 0;
    let mut total: i128 = 0;
    
    
    let mut current_nodes_cycle: HashMap<usize, u128> = HashMap::new();

    println!("{:?}", current_nodes);
    while current_nodes_cycle.values().len() != current_nodes.len() {
//        println!("{:?}", current_nodes);
        
        if instruction_curr >= instructions.len() {
            instruction_curr = 0;
        }

        for h in 0..current_nodes.len() {
            if (current_nodes[h].chars().collect::<Vec<char>>()[2]) == 'Z' {
                if current_nodes_cycle.get(&h).is_none() {
                    current_nodes_cycle.insert(h, total as u128);
                }
            }

            if instructions[instruction_curr] == 'R' {
                let c = m.get(current_nodes[h]).unwrap();
                current_nodes[h] = c.1;
            }
    
            if instructions[instruction_curr] == 'L' {
                let c = m.get(current_nodes[h]).unwrap();
                current_nodes[h] = c.0;
            }
        }
        println!("{:?}", current_nodes);
        total += 1;
        instruction_curr += 1;
    }
    println!("{:?}", current_nodes_cycle);

    let values: Vec<u128> = current_nodes_cycle.values().cloned().collect::<Vec<u128>>();
    let values_factor: u128 = values.iter().fold(1, |x, y| x * y);

    println!("values: {:?} values_factor: {}", values, values_factor);
    println!("Least common multiple: {}", least_common_multiple(values));

    /* brute force 
    while !all_current_on_z(&current_nodes) {
        if instruction_curr >= instructions.len() {
            instruction_curr = 0;
        }

        for h in 0..current_nodes.len() {

            if h == 0 && (current_nodes[h].chars().collect::<Vec<char>>()[2]) == 'Z' {
                println!("{}", total);
            }

            if instructions[instruction_curr] == 'R' {
                let c = m.get(current_nodes[h]).unwrap();
                current_nodes[h] = c.1;
            }
    
            if instructions[instruction_curr] == 'L' {
                let c = m.get(current_nodes[h]).unwrap();
                current_nodes[h] = c.0;
            }
        }

        total += 1;
        // println!("total {} current {:?}", total, current_nodes);
        instruction_curr += 1;    
    }*/
    println!("total {}", total);
}

fn least_common_multiple(values: Vec<u128>) -> u128 {
    
    let mut lcm = values[0];
    for i in 1..values.len() {
        let prev = lcm;
        lcm = least_common_multiple_pair(lcm, values[i]);
        println!("v1: {} v2: {} lcm: {}", prev, values[i], lcm);
    }

    return lcm;
}

fn least_common_multiple_pair(i: u128, j: u128) -> u128 {
    return (i*j)/greatest_common_divisor(i, j);
}

fn greatest_common_divisor(mut m: u128, mut n: u128) -> u128 {
    while m != 0 {
        if m < n {
            swap(&mut m, &mut n);
        }

        m %= n;
    }

    return n;
}

fn day_nine_one() {
    let input = "20 42 73 115 170 240 327 433 560 710 885 1087 1318 1580 1875 2205 2572 2978 3425 3915 4450
    25 39 53 67 81 95 109 123 137 151 165 179 193 207 221 235 249 263 277 291 305
    12 29 76 164 312 559 976 1678 2836 4689 7556 11848 18080 26883 39016 55378 77020 105157 141180 186668 243400";

    let lines = input.split("\n");

    let mut total = 0;

    for line in lines {
        let numbers: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let mut diffs: Vec<Vec<i32>> = vec![numbers];
        let mut current = 0;

        while !diffs[current].iter().all(|v: &i32| *v == 0) {
            let mut v: Vec<i32> = vec![];

            for i in 0..(diffs[current].len() - 1) {
                v.push(diffs[current][i+1] - diffs[current][i]);
            }

            println!("{:?}", v);
            diffs.push(v);
            current += 1;
        }

        for h in 2..diffs.len() + 1 {
            let c = diffs.len() - h;
            let p = diffs.len() - h + 1;
            let new_val = diffs[p][diffs[p].len() - 1] + diffs[c][diffs[c].len() - 1];
            diffs[c].push(new_val);
            println!("qwerljqwerqwer");
        }

        total += diffs[0][diffs[0].len() - 1];
        println!("{:?}", diffs);
    }
    
    println!("total: {}", total);
    
}

fn day_nine_two() {
    let input = "20 42 73 115 170 240 327 433 560 710 885 1087 1318 1580 1875 2205 2572 2978 3425 3915 4450
    25 39 53 67 81 95 109 123 137 151 165 179 193 207 221 235 249 263 277 291 305
    12 29 76 164 312 559 976 1678 2836 4689 7556 11848 18080 26883 39016 55378 77020 105157 141180 186668 243400";

    let lines = input.split("\n");

    let mut total = 0;

    for line in lines {
        let numbers: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let mut diffs: Vec<Vec<i32>> = vec![numbers];
        let mut current = 0;

        while !diffs[current].iter().all(|v: &i32| *v == 0) {
            let mut v: Vec<i32> = vec![];

            for i in 0..(diffs[current].len() - 1) {
                v.push(diffs[current][i+1] - diffs[current][i]);
            }

            println!("{:?}", v);
            diffs.push(v);
            current += 1;
        }

        for h in 2..diffs.len() + 1 {
            let c = diffs.len() - h;
            let p = diffs.len() - h + 1;
            let new_val = diffs[c][0] - diffs[p][diffs[p].len() - 1];
            println!("{} - {}", diffs[c][0], diffs[p][diffs[p].len() - 1]);
            diffs[c].push(new_val);
        }

        total += diffs[0][diffs[0].len() - 1];
        println!("{:?}", diffs);
    }
    
    println!("total: {}", total);
}

fn get_s_location(v: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..v.len() {
        for j in 0..(v[i]).len() {
            if v[i][j] == 'S' {
                return (j, i);
            }
        }
    }

    return (0, 0);
}

fn find_loop(curr: (usize, usize), prev: (usize, usize), v: &Vec<Vec<char>>, res: &mut Vec<(usize, usize)>) -> bool {
    if curr.0 > 0 && prev.0 == curr.0 - 1 {
        // came from the left
        if v[curr.1][curr.0] == '|' 
        || v[curr.1][curr.0] == 'L' 
        || v[curr.1][curr.0] == 'F' 
        || v[curr.1][curr.0] == '.' {
            return false;
        } else if v[curr.1][curr.0] == 'J' {
            if curr.1 == 0 {
                return false;
            }

            if find_loop((curr.0, curr.1 - 1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else if v[curr.1][curr.0] == '7' {
            if curr.1 == v.len() - 1 {
                return false;
            }

            if find_loop((curr.0, curr.1 + 1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else if v[curr.1][curr.0] == '-' {
            if curr.0 == v[0].len() - 1 {
                return false;
            }

            if find_loop((curr.0 + 1, curr.1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else {
            (*res).push(curr);
            return true;
        }
    } else if prev.0 == curr.0 + 1 {
        // came from the right

        if v[curr.1][curr.0] == '|' 
        || v[curr.1][curr.0] == 'J' 
        || v[curr.1][curr.0] == '7' 
        || v[curr.1][curr.0] == '.' {
            return false;
        } else if v[curr.1][curr.0] == 'L' {
            if curr.1 == 0 {
                return false;
            }

            if find_loop((curr.0, curr.1 - 1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else if v[curr.1][curr.0] == 'F' {
            if curr.1 == v.len() - 1 {
                return false;
            }

            if find_loop((curr.0, curr.1 + 1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else if v[curr.1][curr.0] == '-' {
            if curr.0 == 0 {
                return false;
            }

            if find_loop((curr.0 - 1, curr.1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else {
            (*res).push(curr);
            return true;
        }
    } else if prev.1 == curr.1 + 1 {
        // came from the bottom
        if v[curr.1][curr.0] == 'L' 
        || v[curr.1][curr.0] == '-' 
        || v[curr.1][curr.0] == 'J' 
        || v[curr.1][curr.0] == '.' {
            return false;
        } else if v[curr.1][curr.0] == '|' {
            if curr.1 == 0 {
                return false;
            }

            if find_loop((curr.0, curr.1 - 1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else if v[curr.1][curr.0] == 'F' {
            if curr.0 == v[0].len() - 1 {
                return false;
            }

            if find_loop((curr.0 + 1, curr.1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else if v[curr.1][curr.0] == '7' {
            if curr.0 == 0 {
                return false;
            }

            if find_loop((curr.0 - 1, curr.1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else {
            (*res).push(curr);
            return true;
        }
    } else if curr.1 > 0 && prev.1 == curr.1 - 1 {
        // came from the top
        if v[curr.1][curr.0] == 'F' 
        || v[curr.1][curr.0] == '-' 
        || v[curr.1][curr.0] == '7' 
        || v[curr.1][curr.0] == '.' {
            return false;
        } else if v[curr.1][curr.0] == 'L' {
            if curr.0 == v[0].len() - 1 {
                return false;
            }

            if find_loop((curr.0 + 1, curr.1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else if v[curr.1][curr.0] == '|' {
            if curr.1 == v.len() - 1 {
                return false;
            }

            if find_loop((curr.0, curr.1 + 1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else if v[curr.1][curr.0] == 'J' {
            if curr.0 == 0 {
                return false;
            }

            if find_loop((curr.0 - 1, curr.1), curr, v, res) {
                (*res).push(curr);
                return true;
            }
        } else {
            (*res).push(curr);
            return true;
        }
    }

    return false;
}

fn day_ten_one() {

    let input = "F7-FJ77FL|-L.7.FJ77F-7FF|7-F7F|--F7F-F7.-F---7L7FL7L77-7.F-7F-7FF.--.777F|.7.77-F-7F-LJ7F7F77.F.FF-7|-J-F|F--J--7J7.7--7-LF-J7J7.7.-L-FF77.|
    F7-J||7JJ|FLF7.---|7J|7.||7L|-|-|LF7-FJL-|-|F7-J7.7LLJ-|--7LLLL-|F|FF7LLJJF|-LJ.||F-7J.|-JJ|JF|FJ.FLJJ7FL|J-JJ|.|FL|-7JJ|L|J7F-F-|77L7L|LJ7J
    .|77LLJ.F-|.FJ7.FLLJF-J-7-J7.FJ-J-LJ.7-L7|-F|J|F-7..|LFJ7LL7.|.-.F7--L7LL--7J||.F-|-L-F|LJLJFFJ|7FJ.--FJ||L7-F|-77JL|J|.7-F7L..F--J--7-F-F7.";

    let v: Vec<Vec<char>> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| (*x).trim().chars().collect::<Vec<char>>())
        .collect();

    let curr = get_s_location(&v);
    let mut res = vec![];

    println!("{:?}", curr);
    println!("LEFT");
    if curr.0 > 0 && find_loop((curr.0 - 1, curr.1), curr, &v, &mut res) {
        println!("{}", res.len()/2);
    }
    
    println!("RIGHT");
    if find_loop((curr.0 + 1, curr.1), curr, &v, &mut res) {
        println!("{}", res.len()/2);
    }

    println!("TOP");
    if curr.1 > 0 && find_loop((curr.0, curr.1 - 1), curr, &v, &mut res) {
        println!("{}", res.len()/2);
    }

    println!("BOTTOM");
    if  find_loop((curr.0, curr.1 + 1), curr, &v, &mut res) {
        println!("{}", res.len()/2);
    }
}


fn day_ten_two() {
    let input = "F7-FJ77FL|-L.7.FJ77F-7FF|7-F7F|--F7F-F7.-F---7L7FL7L77-7.F-7F-7FF.--.777F|.7.77-F-7F-LJ7F7F77.F.FF-7|-J-F|F--J--7J7.7--7-LF-J7J7.7.-L-FF77.|
    F7-J||7JJ|FLF7.---|7J|7.||7L|-|-|LF7-FJL-|-|F7-J7.7LLJ-|--7LLLL-|F|FF7LLJJF|-LJ.||F-7J.|-JJ|JF|FJ.FLJJ7FL|J-JJ|.|FL|-7JJ|L|J7F-F-|77L7L|LJ7J
    .|77LLJ.F-|.FJ7.FLLJF-J-7-J7.FJ-J-LJ.7-L7|-F|J|F-7..|LFJ7LL7.|.-.F7--L7LL--7J||.F-|-L-F|LJLJFFJ|7FJ.--FJ||L7-F|-77JL|J|.7-F7L..F--J--7-F-F7.";

    let v: Vec<Vec<char>> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| (*x).trim().chars().collect::<Vec<char>>())
        .collect();

    let curr = get_s_location(&v);
    let mut res = vec![];

    println!("{:?}", curr);

    if curr.0 > 0 && find_loop((curr.0 - 1, curr.1), curr, &v, &mut res) {
        println!("LEFT");
        println!("{}", res.len()/2);
    } else if find_loop((curr.0 + 1, curr.1), curr, &v, &mut res) {
        println!("RIGHT");
        println!("{}", res.len()/2);
    } else if curr.1 > 0 && find_loop((curr.0, curr.1 - 1), curr, &v, &mut res) {
        println!("{}", res.len()/2);
    } else if  find_loop((curr.0, curr.1 + 1), curr, &v, &mut res) {
        println!("{}", res.len()/2);
    }

    let res_set: HashSet<(usize, usize)> = HashSet::from_iter(res.iter().cloned());

    let mut total = 0;
    for j in 0..v.len(){
        for i in 0..v[0].len() {
            // silly raycast solution
            let mut x = i;
            let mut n: usize = 0;

            if !res_set.contains(&(i, j)) {
                while x < v[0].len() {
                    if (v[j][x] == '|' || v[j][x] == 'J' || v[j][x] == 'L') && res_set.contains(&(x, j)) {
                        n += 1;
                    }
                    
                    x += 1;
                }

                if n != 0 && n % 2 != 0 {
                    println!("X {}, Y {} N {}", i, j, n);
                    total += 1;
                }
            }

        }
    }

    println!("TOTAL {}", total);
}