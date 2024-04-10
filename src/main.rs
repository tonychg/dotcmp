use clap::Parser;
use std::{collections::HashMap, fs};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Target dotenv
    #[arg(short, long, value_name = "TARGET")]
    target: String,

    /// Source dotenv
    #[arg(short, long, value_name = "SOURCE")]
    source: String,

    #[arg(short, long)]
    missing: bool,

    #[arg(short, long)]
    difference: bool,
}

#[derive(Debug, Default)]
struct Dotenv {
    pub path: String,
    pub values: HashMap<String, String>,
}

impl Dotenv {
    fn from_path(path: &str) -> Self {
        Self {
            path: path.to_string(),
            values: Self::serialize(
                &fs::read_to_string(path).expect("Something went wrong reading the file"),
            ),
        }
    }

    fn serialize(data: &str) -> HashMap<String, String> {
        data.split('\n')
            .filter(|line| !line.is_empty() && !line.starts_with('#') && line.contains('='))
            .map(|line| {
                let variable: Vec<&str> = line.splitn(2, '=').collect();
                let label = variable.first().unwrap().to_string();
                let value = match variable.get(1) {
                    Some(value) => value.to_string(),
                    None => "".to_string(),
                };
                (label, value)
            })
            .collect()
    }

    fn compare(&self, dotenv: &Dotenv, missing: bool, difference: bool) {
        if missing {
            println!("Not in {}", self.path);
        }
        for (label, value) in dotenv.values.clone().into_iter() {
            match self.values.get(&label) {
                Some(target) => {
                    if target != &value && difference {
                        println!("{}: {} => {}", label, target, value);
                    }
                }
                None => {
                    if missing {
                        println!("{}={}", label, value);
                    }
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let target = Dotenv::from_path(&args.target);
    let source = Dotenv::from_path(&args.source);
    target.compare(&source, args.missing, args.difference);
}
