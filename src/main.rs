#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar.pest"] // Path to your .pest file
pub struct MyParser;

#[derive(Debug, PartialEq)]
enum Person {
    Dot,
    X,
    Percent,
}

#[derive(Debug, PartialEq)]
enum Group {
    ColonColon,
    HashHash,
}

#[derive(Debug, PartialEq)]
enum Time {
    Past,
    Present,
    Future,
}

#[derive(Debug, PartialEq)]
enum ArrowSign {
    Positive,
    Negative,
    None,
}

#[derive(Debug, PartialEq)]
enum Certainty {
    Certain,
    Uncertain,
    Neutral,
}

#[derive(Debug, PartialEq)]
enum Nothing {
    EmptyBrackets,
}

#[derive(Debug, PartialEq)]
enum Something {
    SingleExclamation,
    DoubleExclamation,
    TripleExclamation,
}

#[derive(Debug, PartialEq)]
enum Everything {
    BracketX,
}

#[derive(Debug, PartialEq)]
enum Thinks {
    DotDotDot,
    DotMinusDot,
}


#[derive(Debug, PartialEq)]
enum Name {
    PersonNode(Person),
    GroupNode(Group),
}

#[derive(Debug, PartialEq)]
enum Item {
    NothingNode(Nothing),
    SomethingNode(Something),
    EverythingNode(Everything),
}

#[derive(Debug, PartialEq)]
enum Noun {
    NameNode(Name),
    ItemNode(Item),
}

#[derive(Debug, PartialEq)]
enum Acts {
    Arrow {
        invert: bool,
        sign: ArrowSign,
    },
}

#[derive(Debug, PartialEq)]
enum Says {
    Statement {
        certainty: Certainty,
        invert: bool,
        time: Option<Time>,
    },
}

#[derive(Debug, PartialEq)]
enum Concept {
    CompositeStatement(Box<CompositeStatement>),
}

#[derive(Debug, PartialEq)]
enum Action {
    Perform {
        subject: Box<NounOrConcept>,
        acts: Acts,
        object: Box<NounOrConcept>,
    },
}

#[derive(Debug, PartialEq)]
enum Proximity {
    Near(Box<NounOrConcept>, Box<NounOrConcept>),
}

#[derive(Debug, PartialEq)]
enum Made {
    Creation(Box<NounOrConcept>, Box<NounOrConcept>),
}

#[derive(Debug, PartialEq)]
enum Relationship {
    ActionNode(Action),
    ProximityNode(Proximity),
    MadeNode(Made),
}

#[derive(Debug, PartialEq)]
enum ExtendedConcept {
    RelationshipNode(Relationship),
    // Additional fields as necessary
}

#[derive(Debug, PartialEq)]
enum Statement {
    RelationshipNode(Relationship),
    NounNode(Noun),
    ExtendedConceptNode(ExtendedConcept),
}

#[derive(Debug, PartialEq)]
enum CompositeSays {
    Expression {
        speaker: Name,
        says: Says,
        statement: Box<CompositeStatement>,
    },
}

#[derive(Debug, PartialEq)]
enum CompositeThinks {
    Expression {
        thinker: Name,
        thinks: Thinks,
        statement: Box<CompositeStatement>,
    },
}

#[derive(Debug, PartialEq)]
enum CompositeStatement {
    SaysNode(CompositeSays),
    ThinksNode(CompositeThinks),
    StatementNode(Statement),
}

#[derive(Debug, PartialEq)]
enum MultipleStatements {
    Statements(Vec<CompositeStatement>),
}

#[derive(Debug, PartialEq)]
enum TopLevel {
    Program(MultipleStatements),
}

// Helper enum to simplify handling of nodes that can be either Noun or Concept
#[derive(Debug, PartialEq)]
enum NounOrConcept {
    Noun(Noun),
    Concept(Concept),
}

impl MyParser {
    fn parse_top_level(pair: Pair<Rule>) -> Result<TopLevel, String> {
        let inner_rules = pair.into_inner();

        for pair in inner_rules {
            match pair.as_rule() {
                Rule::multiple_statements => {
                    let result = Self::parse_multiple_statements(pair)?;
                    return Ok(TopLevel::Program(MultipleStatements::Statements(result)));
                },
                _ => return Err(format!("Unexpected rule in top level: {:?}", pair.as_rule())),
            }
        }
        Err(format!("Error?????"))
    }

    fn parse_multiple_statements(pair: Pair<Rule>) -> Result<Vec<CompositeStatement>, String> {
        let inner_rules = pair.into_inner();
        let mut statements = Vec::new();

        for pair in inner_rules {
            match pair.as_rule() {
                Rule::composite_statement => {
                    let statement = Self::parse_composite_statement(pair)?;
                    statements.push(statement);
                },
                Rule::multiple_statements => {
                    unimplemented!()
                }
                _ => return Err(format!("Unexpected rule in multiple_statements: {:?}", pair.as_rule())),
            }
        }

        Ok(statements)
    }

    fn parse_composite_statement(pair: Pair<Rule>) -> Result<CompositeStatement, String> {
        let inner_rules = pair.into_inner();
        
        if inner_rules.len() != 1 {
            return Err(format!("Composite Statement must contain exactly one rule!"));
        }
        let pair = inner_rules.peek().expect("Expected at least one rule!");

        match pair.as_rule() {
            Rule::composite_says => {
                // Parse composite_says into CompositeSays node
                unimplemented!()
            },
            Rule::composite_thinks => {
                // Parse composite_thinks into CompositeThinks node
                return Ok(CompositeStatement::ThinksNode(Self::parse_composite_thinks(pair)?));
            },
            Rule::statement => {
                // Parse statement into Statement node
                return Ok(CompositeStatement::StatementNode(Self::parse_statement(pair)?));
            },
            _ => Err(format!("Unexpected rule in composite_statement: {:?}", pair)),
        }
    }

    fn parse_statement(pair: Pair<Rule>) -> Result<Statement, String> {
        let inner_rules = pair.into_inner();
        
        if inner_rules.len() != 1 {
            return Err(format!("Statement must contain exactly one rule!"));
        }
        let pair = inner_rules.peek().expect("Expected at least one rule!");

        match pair.as_rule() {
            Rule::extended_relationship => {
                // Parse composite_says into CompositeSays node
                unimplemented!()
            },
            Rule::noun => {
                // Parse composite_thinks into CompositeThinks node
                return Ok(Statement::NounNode(Self::parse_noun(pair)?));
            },
            Rule::extended_concept => {
                // Parse statement into Statement node
                unimplemented!()
            },
            _ => Err(format!("Unexpected rule in composite_statement: {:?}", pair)),
        }
        
    }

    fn parse_composite_thinks(pair: Pair<Rule>) -> Result<CompositeThinks, String> {
        let mut inner_rules = pair.into_inner();
        
        if inner_rules.len() != 3 {
            return Err(format!("Expected three rules!"));
        }

        let name_rule = inner_rules.nth(0).expect("Expected one value.");
        let thinks_rule = inner_rules.nth(0).expect("Expected two values.");
        let composite_statement_rule = inner_rules.nth(0).expect("Expected three values.");

        let thinker = Self::parse_name(name_rule)?;
        let thinks = Self::parse_thinks(thinks_rule.as_str())?;
        let statement = Self::parse_composite_statement(composite_statement_rule)?;

        return Ok(CompositeThinks::Expression{thinker, thinks, statement: Box::new(statement)}); 
    }
    
    fn parse_noun(pair: Pair<Rule>) -> Result<Noun, String> {
        let inner_rules = pair.into_inner();
        
        if inner_rules.len() != 1 {
            return Err(format!("Noun must contain exactly one rule!"));
        }
        let pair = inner_rules.peek().expect("Expected at least one rule!");

        match pair.as_rule() {
            Rule::name => Ok(Noun::NameNode(Self::parse_name(pair)?)),
            Rule::item => Ok(Noun::ItemNode(Self::parse_item(pair)?)),
            _ => Err(format!("Unexpected rule in noun: {:?}", pair)),
        }
    }
    
    fn parse_item(pair: Pair<Rule>) -> Result<Item, String> {
        let inner_rules = pair.into_inner();
        
        if inner_rules.len() != 1 {
            return Err(format!("Noun must contain exactly one rule!"));
        }
        let pair = inner_rules.peek().expect("Expected at least one rule!");

        match pair.as_str() {
            "[]" => Ok(Item::NothingNode(Nothing::EmptyBrackets)),
            "[!]" => Ok(Item::SomethingNode(Something::SingleExclamation)),
            "[!!]" => Ok(Item::SomethingNode(Something::DoubleExclamation)),
            "[!!!]" => Ok(Item::SomethingNode(Something::TripleExclamation)),
            "[X]" => Ok(Item::EverythingNode(Everything::BracketX)),
            _ => Err(format!("Unexpected rule in noun: {:?}", pair)),
        }
    }
    
    fn parse_name(pair: Pair<Rule>) -> Result<Name, String> {
        let inner_rules = pair.into_inner();
        
        if inner_rules.len() != 1 {
            return Err(format!("Name must contain exactly one rule!"));
        }
        let pair = inner_rules.peek().expect("Expected at least one rule!");

        match pair.as_rule() {
            Rule::person => {
                // Parse composite_says into CompositeSays node
                return Ok(Name::PersonNode(Self::parse_person(pair.as_str())?));
            },
            Rule::group => {
                // Parse composite_thinks into CompositeThinks node
                unimplemented!()
            },
            _ => Err(format!("Unexpected rule in name: {:?}", pair)),
        }
    }
    
    fn parse_thinks(thinks: &str) -> Result<Thinks, String> {
        match thinks {
            "..." => Ok(Thinks::DotDotDot),
            ".-." => Ok(Thinks::DotMinusDot),
            _ => Err(format!("Unexpected value in thinks!")),
        }
    }
    
    fn parse_person(person: &str) -> Result<Person, String> {
        match person {
            "." => Ok(Person::Dot),
            "x" => Ok(Person::X),
            "%" => Ok(Person::Percent),
            _ => Err(format!("Unexpected value in person!")),
        }
    }

    // Methods for parsing other constructs will go here...
}

fn parse_language(input: &str) -> Result<(), pest::error::Error<Rule>> {
    let pairs = MyParser::parse(Rule::top_level, input)?;
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
    let parse_result = MyParser::parse(Rule::top_level, input);
    match parse_result {
        Ok(pairs) => {
            for pair in pairs {
                match MyParser::parse_top_level(pair) {
                    Ok(ast) => println!("{:?}", ast),
                    Err(e) => eprintln!("Error parsing top level: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Parsing error: {}", e),
    }
}

