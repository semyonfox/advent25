use std::fs::read_to_string;

pub fn day2_1() {
    let mut invalid: i64 = 0;
    println!("day2");

    // read in
    let message: String = read_to_string("src/two/input.txt").expect("Failed to read file");
    // csv to array if needed
    let mut ranges: Vec<&str> = message.split(',').collect();

    // iterate over intermediate values
    for r in &mut ranges {
        // for each:
        // break at -
        // let bounds: Vec<&str> = r.split('-').collect();
        // let lower: &str = bounds[0];
        // let upper: &str = bounds[1];
        // the better way
        let (lower, upper) = r.split_once('-').expect("expected `lower-upper`");

        // numerify them
        let lower_int: i64 = lower
            .trim()
            .parse::<i64>()
            .expect("Failed to parse lower bound");
        let upper_int: i64 = upper
            .trim()
            .parse::<i64>()
            .expect("Failed to parse upper bound");

        // start loop with bounds
        for i in lower_int..=upper_int {
            // check if length is even!!!!!!!!!!!!!
            // im stupid
            // split in half\
            // easier to stringify
            let number: String = i.to_string();
            if number.len() % 2 != 0 {
                continue;
            }
            let mid = number.len() / 2;
            let first_half = &number[..mid];
            let second_half = &number[mid..];
            // compare halves

            //sum you idiot
            if first_half == second_half {
                invalid += i;
            }
        }
    }

    println!("invalid: {}", invalid);
}

pub fn day2_2() {
    let mut num_invalid: i64 = 0;
    println!("day2");

    // read in
    let message: String = read_to_string("src/two/input.txt").expect("Failed to read file");
    // csv to array if needed
    let mut ranges: Vec<&str> = message.split(',').collect();

    // iterate over intermediate values
    for r in &mut ranges {
        // for each:
        // break at -

        // the better way
        let (lower, upper) = r.split_once('-').expect("expected `lower-upper`");

        // numerify them for the for loop
        let lower_int: i64 = lower
            .trim()
            .parse::<i64>()
            .expect("Failed to parse lower bound");
        let upper_int: i64 = upper
            .trim()
            .parse::<i64>()
            .expect("Failed to parse upper bound");

        // start loop with bounds
        for num in lower_int..=upper_int {
            // get the factors of i
            // store them and sort (high to low)
            // subdivide the i string to the array,
            // iterating over the factors high to low
            // if match found invalid ++ and break (else if)

            let factors: Vec<i64> = factorise(num.to_string().len() as i64); // .len is the human-readable length (assuming no dodgy more than normal byte chars) e.g. foo = 3 NOT 2
            let number: String = num.to_string();
            let digits: Vec<char> = number.chars().collect();
            'outer: for f in &factors {
                // get breakpoint every f places

                let parts: Vec<_> = digits.chunks(*f as usize).collect();
                if parts.windows(2).all(|w| w[0] == w[1]) {
                    num_invalid += num;
                    break 'outer;
                }
            }
            // need to break to here when found
        }
    }

    println!("invalid: {}", num_invalid);
}

pub fn factorise(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut d = 1_i64;

    while d * d <= n {
        if n % d == 0 {
            if d != n {
                factors.push(d);
            }
            let other = n / d;
            if other != d && other != n {
                factors.push(other); // ignores itself
            }
        }
        d += 1;
    }

    factors.sort_unstable();
    factors.dedup(); // remove any remaining duplicates just in case
    factors.reverse();
    factors
}
