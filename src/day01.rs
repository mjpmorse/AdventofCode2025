use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;

fn read_number(input: &PathBuf) -> anyhow::Result<Vec<i32>> {
    // Read the input document to get the new safe combination
    // Output will be a vector of integers. Positive integers are to the right
    // Negative integers are to the lft

    // open the file at the given path
    let file = File::open(input)?;
    // create a buffered reader to read the file
    let reader = std::io::BufReader::new(file);
    // Create a vector to store the numbers, with an initial capacity of 500
    let mut list1: Vec<i32> = Vec::with_capacity(500);

    // iterate over the lines of the file
    for line in reader.lines() {
        // Read the line, returning an error if it fails
        let line = line?;
        // Split off the first character of the line which will be L or R
        let (direction, nbr) = line.split_at(1);
        let pos_direction = String::from('R');
        let sign: i32 = if direction == pos_direction {1} else {-1};
        // Parse the line as an i32 and push it to the list if successful
        if let Ok(parsed) = nbr.parse::<i32>() {
            // switch positive or negative based on direction being R or L
            list1.push(sign * parsed);
        }
    }
    // return the list of numbers
    Ok(list1)
}

fn mod_the_password(nbr: &i32, dial_size: &i32) -> i32{
    // If the number is less than 0, or greater than 99 we need to mod it.
    // example if 100 we need to return 0 because that is one click more than 99
    // example if -5 we need to return 95 because that is 5 clicks less than 0

    // nbr % (dial_size + 1) <- this does not do mod correct like I wanted
    nbr.rem_euclid(dial_size + 1)
}

fn floor_division(nbr: &i32, dial_size: &i32) -> i32{
    // If the number is less than 0, or greater than 99 we need to mod it.
    // example if 100 we need to return 0 because that is one click more than 99
    // example if -5 we need to return 95 because that is 5 clicks less than 0

    // nbr % (dial_size + 1) <- this does not do mod correct like I wanted
    nbr.div_euclid(dial_size + 1)
}

pub fn part_a_old(input: &PathBuf, dial_size: &i32, starting_point: i32) -> anyhow::Result<i32> {
    // Read the list of numbers from the input file
    let mut combo_list = read_number(input)?;
    // insert starting point
    combo_list.insert(0, starting_point);

    // Take the partial sums up each element in the array
    let partial_sum: Vec<i32> = combo_list.iter().scan(
        0, |acc, &x| {
            *acc += x;
            Some(*acc)
        }
    ).collect();
    let mod_sum: Vec<i32> = partial_sum.iter().map(|x| mod_the_password(x, dial_size)).collect();
    // count the 0s in partial_sum
    let zero_count: i32 = mod_sum.iter().filter(|&x| x == &0).count() as i32;
    Ok(zero_count)

}


pub fn part_a(input: &PathBuf, dial_size: &i32, starting_point: i32) -> anyhow::Result<i32> {
    // Read the list of numbers from the input file
    let combo_list = read_number(input)?;

    let mut current_value: i32 = starting_point;
    let mut next_value: i32;
    let mut zero_count: i32 = 0;
    let mut modulo_value;
    // let's try like this going around the dial
    for x in combo_list.iter(){
        next_value = current_value + x;
        // the modulo value gives us the current dial position
        modulo_value = mod_the_password(&next_value, dial_size);

        // This is the part-A solution
        if modulo_value == 0 {zero_count += 1}

        println!("After applying rotation {} we are at {} with our total zero count is now {}", x, modulo_value, zero_count);
        current_value = modulo_value;

    }

    Ok(zero_count)

}

pub fn part_b(input: &PathBuf, dial_size: &i32, starting_point: i32) -> anyhow::Result<i32> {
    // Read the list of numbers from the input file
    let combo_list = read_number(input)?;

    let mut current_value: i32 = starting_point;
    let mut next_value: i32;
    let mut zero_count: i32 = 0;
    let mut nbr_rotations: i32;
    let mut modulo_value;
    let mut previous_value: i32;
    // let's try like this going around the dial
    for x in combo_list.iter(){
        previous_value = current_value;
        next_value = current_value + x;
        // the modulo value gives us the current dial position
        modulo_value = mod_the_password(&next_value, dial_size);
        // we also need to calculate how many times we went around
        // if we passed zero.
        // we pass zero if next_value < 0 or next_value > 99
        // also make sure we don't count when we click over off a previous 0
        if next_value <= 0 || next_value >= *dial_size {
            nbr_rotations = floor_division(&next_value, dial_size).abs();
            // here we account for that 100 // 100 = 1, but we are not passing zero.
            // we are sitting on it
            if modulo_value == 0 {nbr_rotations -= 1}
            // also if we are already at zero we need to make sure that we don't count that
            if previous_value == 0 {nbr_rotations -= 1}
            // validate that we are never removing rotations
            if nbr_rotations < 0 {println!("rotations < 0"); nbr_rotations = 0;}

        }
        else{
            nbr_rotations = 0
        }
        // This is the part-A solution
        if modulo_value == 0 {zero_count += 1}
        // For part B we also add in the rotations through the point
        zero_count += nbr_rotations;

        println!("After applying rotation {} we are at {} with our total zero count is now {}", x, modulo_value, zero_count);
        current_value = modulo_value;

    }

    Ok(zero_count)

}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_mod(){
        let nbr = &100;
        let dial_size = &99;
        let result = mod_the_password(nbr, dial_size);
        assert_eq!(result, 0)

    }

    #[test]
    fn test_part_a() {
        // Arrange
        let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("input")
            .join("day01_example.txt");
        let dial_size = &99;
        let starting_point = 50;

        // Act
        let result = part_a(
            &input_path, dial_size, starting_point
        ).expect("part_a failed"); // Extract the `i32`

        // Assert
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_b() {
        // Arrange
        let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("input")
            .join("day01_example.txt");
        let dial_size = &99;
        let starting_point = 50;

        // Act
        let result = part_b(
            &input_path, dial_size, starting_point
        ).expect("part_b failed"); // Extract the `i32`

        // Assert
        assert_eq!(result, 6);
    }

}