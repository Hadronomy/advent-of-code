use miette::{miette, LabeledSpan, Result, Severity};

fn main() -> Result<()> {
    println!("Hello, world!");
    let x = 1;
    let y = 2;
    let source = "(2 + 2".to_string();
    let report = miette!(
        // Those fields are optional
        severity = Severity::Error,
        code = "expected::rparen",
        help = "always close your parens",
        labels = vec![LabeledSpan::at_offset(5, "here")],
        url = "https://example.com",
        // Rest of the arguments are passed to `format!`
        // to form diagnostic message
        "expected closing ')'"
    )
    .with_source_code(source);
    Err(report)
}
