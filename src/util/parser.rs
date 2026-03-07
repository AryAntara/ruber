use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/core.pest"]
pub struct ScriptParser;
