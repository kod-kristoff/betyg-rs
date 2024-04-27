use clap::{value_parser, Arg, Command};
use csv::Reader;
use std::path::PathBuf;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("Error occurred: {:?}", err);
        std::process::exit(1);
    }
}

fn try_main() -> eyre::Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("räkna", submatches)) => {
            let file_path = submatches.get_one::<PathBuf>("file").unwrap();
            let mut rdr = Reader::from_path(file_path)?;
            let mut num_grades = 0;
            let mut sum_grades = 0.0;
            for result in rdr.deserialize() {
                let row: Subject = result?;
                println!("{:?}", row);
                num_grades += 1;
                sum_grades += row.grade_as_num();
            }
            println!(
                "Meritvärde: {} (baserat på {} betyg)",
                sum_grades, num_grades
            );
        }
        _ => todo!(),
    };
    Ok(())
}

fn cli() -> Command {
    Command::new("betyg").subcommand(
        Command::new("räkna").arg(
            Arg::new("file")
                .short('f')
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        ),
    )
}

#[derive(Debug, serde::Deserialize)]
pub struct Subject {
    subject: String,
    grade: String,
}

impl Subject {
    pub fn grade_as_num(&self) -> f32 {
        match self.grade.as_str() {
            "A" => 20.0,
            "B" => 17.5,
            "C" => 15.0,
            "D" => 12.5,
            "E" => 10.0,
            "F" => 0.0,
            _ => panic!("invalid grade"),
        }
    }
}
