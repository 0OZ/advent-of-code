mod input;

fn main(){
    let mut sum = 0;

    for line in input::get_input().lines() {
        calc_sum_for_numbers(line, &mut sum);
    }
    println!("{}", sum);
}


#[allow(dead_code)]
fn calc_sum_for_numbers(line: &str, sum: &mut i32) {
    // count numbers of line
    let number_string = line.chars().filter(|c| c.is_numeric()).collect::<String>();
    let number_count = number_string.len();
        
    match number_count {
        1 => {
            let numbers = format!("{}{}", number_string, number_string);
            println!("1 number: {}", numbers);
            *sum += numbers.parse::<i32>().unwrap();
        },
        _ => {
            // println!("line: {}", line);
            let first_char = number_string.chars().next();
            let last_char = number_string.chars().last();

            let first_last_char = first_char.unwrap().to_string() + &last_char.unwrap().to_string();
            let numbers:i32 = first_last_char.parse().unwrap();
            println!("n numbers: {}", numbers); 
            *sum += numbers;
        }
    }
}

