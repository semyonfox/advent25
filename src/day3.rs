use std::fs::read_to_string;

fn day3() {
    println!("day3");
    //read in each line from file
    // go over all the lines,

    // starting at 9, if no 9 found, move to 8, etc
    // once i have the highest
    // go to the first one of those in the String array ( more options)
    // from there on
    // repeat that search for that number, and then that number -1 ..0
    // add to a variable - then print it at the end

    let mut max_joltage: i32 = 0;

    let message: String = read_to_string("src/three/input.txt").expect("Failed to read file");
    for (i, line) in message.lines().enumerate() {
        let mut highest_found: i32 = 9;
        let mut local_highest: i32 = 0;

        println!("highest found start: {}", highest_found);
        println!("highest found local: {}", local_highest);
        for (i, c) in line.chars().enumerate() {
            // iterating over the characters in the line, get index too
            if let Some(d) = c.to_digit(10) {
                // now working on characters
                // if i can convert the c( character) to digit base 10 do it
                let d = d as i32; // convert it to i32 (from u32)
                if d == highest_found {
                    // now just compare the digit
                    // if matches
                    local_highest += highest_found * (10);
                    println!("found first digit: {}", highest_found);
                    println!("new local: {}", local_highest);
                    highest_found = 9; // reset highest possible

                    // then i move in, looking for the second digit
                    for c2 in line.chars().skip(i) {
                        // in each character of what remains

                        if let Some(d2) = c2.to_digit(10) {
                            // char -> digit
                            // same again
                            if d2 as i32 == highest_found {
                                local_highest += highest_found;
                                break;
                            } else {
                                highest_found -= 1;
                            }
                        }
                    }
                } else {
                    highest_found -= 1;
                }
            }
        }
        println!("highest found (local): {}", local_highest);
        max_joltage += local_highest;
    }
    println!("joltage: {}", max_joltage);
}

fn ff() {
    //read in each line from file
    // go over all the lines,

    // starting at 9, if no 9 found, move to 8, etc
    // once i have the highest
    // go to the first one of those in the String array ( more options)
    // from there on
    // repeat that search for that number, and then that number -1 ..0
    // add to a variable - then print it at the end

    println!("day3");
    let mut max_joltage: i32 = 0;
    let message: String = read_to_string("src/three/input.txt").expect("Failed to read file");
}
