use {{crate_name}}::part1;

fn main() {
    let input = include_str!("input.txt");
    let result = part1::process(input);
    println!("Result: {}", result.expect("should have a result"));
}
