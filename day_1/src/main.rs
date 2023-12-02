mod input;

fn main() {
    let mut sum = 0;
    for line in input::get_input().lines() {
        calc_sum_for_numbers(line, &mut sum);
    }
    println!("-------------------");
    
    let mut test_sum = 0;
    for line in input::get_test_input().lines() {
        calc_sum_for_numbers(line, &mut test_sum);
    }

    println!("{}", sum);
    println!("{}", test_sum);
}

fn calc_sum_for_numbers(line: &str, sum: &mut i32) {
    // replace words with numbers
    let transformed_line = line
        .replace("one", "o1ne")
        .replace("two", "t2wo")
        .replace("three", "th3ree")
        .replace("four", "fo4ur")
        .replace("five", "fi5ve")
        .replace("six", "s6ix")
        .replace("seven", "se7ven")
        .replace("eight", "ei8ght")
        .replace("nine", "ni9ne");
    // 6oneighthlf
    // 61on8eighthlf

    let number_string = transformed_line
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

    let number_count = number_string.len();
    println!("from {} ->{}", line, transformed_line);
    println!("as number: {}", number_string);

    // if number_count 1 build pair of numbers
    match number_count {
        1 => {
            let numbers = format!("{}{}", number_string, number_string);
            println!("1 number: {}", numbers);
            *sum += numbers.parse::<i32>().unwrap();
        }
        _ => {
            // println!("line: {}", line);
            let first_char = number_string.chars().next();
            let last_char = number_string.chars().last();

            let first_last_char = first_char.unwrap().to_string() + &last_char.unwrap().to_string();
            let numbers: i32 = first_last_char.parse().unwrap();
            println!("n numbers: {}", numbers);
            *sum += numbers;
        }
    }
}
