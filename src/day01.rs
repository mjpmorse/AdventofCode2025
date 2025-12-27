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


fn radians(x: &f64, period: &f64) -> f64{
    x * period
}
fn sin(x: &f64, period: &f64) -> f64{

    (radians(x, period)).sin()
}

fn cos(x: &f64, period: &f64) -> f64{

    (radians(x, period)).cos()
}




pub fn part_a(input: &PathBuf, dial_size: &i32, starting_point: i32) -> anyhow::Result<i32> {
    // Let's try this again mapping to cos. That way we don't have to worry about odd pis.

    let period: f64 = 2. * std::f64::consts::PI / (* dial_size as f64 + 1.);
    let machine_error: f64 = 4. * f64::EPSILON ; // error from the sin function
    // Read the list of numbers from the input file
    let combo_list = read_number(input)?;

    // Starting from 50, we track around the circle
    let partial_sum: Vec<i32> = combo_list.iter().scan(
        starting_point, |acc, &x| {
            *acc += x;
            Some(*acc)
        }
    ).collect();

    // Project onto the circle
    let cos_x: Vec<f64> = partial_sum.iter().map(
        |x| cos(
            &(*x as f64), &period
        )
    ).collect();


    let zero_count: i32 = cos_x.iter().filter(
        |&x| &(x - 1.).abs() <= &machine_error).count() as i32;

    Ok(zero_count)
}

pub fn part_b(input: &PathBuf, dial_size: &i32, starting_point: i32) -> anyhow::Result<i32> {

    // Read the list of numbers from the input file
    let mut combo_list = read_number(input)?;
    combo_list.insert(0, starting_point);

    // Starting from 50, we track around the circle
    let partial_sum: Vec<i32> = combo_list.iter().scan(
        0, |acc, &x| {
            *acc += x;
            Some(*acc)
        }
    ).collect();

    // look at the interval [x0, x1) for each pair in the partial sum map.
    // Again, using cosine so that the function is symmetric, we pass 0 everytime we go a
    // half period. The number of half periods on the interval [x0, x1) is
    // n = (x1 - x0 - 1) / P
    let mut zero_count: i32 = 0;
    for window in partial_sum.windows(2) {
        let start: f32 = window[0] as f32;
        let end: f32 = (window[0] - 1) as f32;
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
    fn test_part_a_sample() {
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
    fn test_part_a() {
        // Arrange
        let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("input")
            .join("day01.txt");
        let dial_size = &99;
        let starting_point = 50;

        // Act
        let result = part_a(
            &input_path, dial_size, starting_point
        ).expect("part_a failed"); // Extract the `i32`

        // Assert
        assert_eq!(result, 1118);
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