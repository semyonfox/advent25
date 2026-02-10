use std::fs::read_to_string;

pub fn day1_1() {
    let mut zeros: i32 = 0;
    println!("day1 p1");
    let mut dial: i32 = 50;
    let message: String = read_to_string("src/one/input.txt").expect("Failed to read file");

    // the dial is 1-99
    // count the zeros

    // parse each line
    //char at 0 = r or l
    // and then get following numbers u to linebreak
    // dial + or - numbers trailing

    for line in message.lines() {
        let direction: char = line.chars().next().expect("Empty line");
        let number: i32 = line[1..line.len()]
            .trim()
            .parse::<i32>()
            .expect("Failed to parse number");

        if direction == 'R' {
            dial += number;
            dial = (dial).rem_euclid(100);
            if dial == 0 {
                zeros += 1;
            }
        } else if direction == 'L' {
            dial -= number;
            dial = (dial).rem_euclid(100);
            if dial == 0 {
                zeros += 1;
            }
        }
    }
    println!("dial: {}", dial);
    println!("zeros: {}", zeros);
}

// =============================================
pub fn day1_2() {
    // essentially count how many times we pass 0
    let mut zeros: i32 = 0;
    println!("day1 p2");
    let mut dial: i32 = 50;
    let message: String = read_to_string("src/one/input.txt").expect("Failed to read file");

    // the dial is 1-99
    // count the zeros

    // parse each line
    //char at 0 = r or l
    // and then get following numbers u to linebreak
    // dial + or - numbers trailing

    for line in message.lines() {
        let direction: char = line.chars().next().expect("Empty line");
        let number: i32 = line[1..line.len()]
            .trim()
            .parse::<i32>()
            .expect("Failed to parse number");

        if direction == 'R' {
            for _ in 0..number {
                dial += 1;
                dial = (dial).rem_euclid(100);
                if dial == 0 {
                    zeros += 1;
                }
            }
        } else if direction == 'L' {
            for _ in 0..number {
                dial -= 1;
                dial = (dial).rem_euclid(100);
                if dial == 0 {
                    zeros += 1;
                }
            }
        }
    }
    println!("dial: {}", dial);
    println!("zeros: {}", zeros);
}
