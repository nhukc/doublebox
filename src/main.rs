#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] // Path to your .pest file
pub struct LanguageParser;

fn parse_language(input: &str) -> Result<(), pest::error::Error<Rule>> {
    let pairs = LanguageParser::parse(Rule::top_level, input)?;
    println!("{:?}", pairs);
    // Process the pairs, building a syntax tree or directly interpreting them
    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                // Process a statement
            },
            // Handle other rules...
            _ => {}
        }
    }
    Ok(())
}

// Main function for testing
fn main() {
    let input = ". ... {x};\nDONE"; // Your input string here
    match parse_language(input) {
        Ok(_) => println!("Parsing successful!"),
        Err(e) => println!("Error: {}", e),
    }
}

