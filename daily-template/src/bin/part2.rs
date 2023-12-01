use {{crate_name}}::part2;

fn main() {
    let input = include_str!("input.txt");
    let result = part2::process(input);
    println!("Result: {}", result.expect("should have a result"));
}
