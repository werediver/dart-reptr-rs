use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "dart.pest"]
pub(crate) struct DartParser;

#[cfg(test)]
mod tests {
    use pest::{error::Error, iterators::Pairs};

    use super::*;

    #[test]
    fn basic() -> Result<(), Error<Rule>> {
        let pairs: Pairs<'_, Rule> = DartParser::parse(Rule::ClassDeclaration, r"class Record")?;
        for pair in pairs {
            println!("{}", pair);
        }
        Ok(())
    }
}
