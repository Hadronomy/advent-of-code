use miette::{miette, LabeledSpan, Result, Severity};

fn main() -> Result<()> {
    println!("Hello, world!");
    let source = "fn main() {
    println!(\"Hello, world!\";
    hello
    hello
    hello
    hello
    hello
    hello
    hello
";
    let report = miette!(
        // Those fields are optional
        severity = Severity::Error,
        code = "expected::rparen",
        help = "always close your parens",
        labels = vec![LabeledSpan::at_offset(40, "here")],
        url = "https://example.com",
        // Rest of the arguments are passed to `format!`
        // to form diagnostic message
        "expected closing ')'"
    )
    .with_source_code(source);
    Err(report)
}
