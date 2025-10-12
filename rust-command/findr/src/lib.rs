use anyhow::Result;
use crate::EntryType::*;
use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};
use regex::Regex;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    #[arg(
        short('n'),
        long("name"),
        value_name = "NAME",
        value_parser(Regex::new),
        action(ArgAction::Append),
        num_args(0..)
    )]
    names: Vec<Regex>,

    #[arg(
        short('t'),
        long("type"),
        value_name = "TYPE",
        value_parser(clap::value_parser!(EntryType)),
        action(ArgAction::Append),
        num_args(0..)
    )]
    entry_type: Vec<EntryType>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        todo!()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        todo!()
    }
}

fn run(args: Args) -> Result<()> {
    Ok(())
}