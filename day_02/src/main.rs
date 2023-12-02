mod input;

fn main() {
    for line in input::get_input().lines() {
        println!("{}", line.trim());
    }
}
