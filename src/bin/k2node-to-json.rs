use anyhow::Result;
use clap::{App, Arg};
use std::fs;
use ue_object_reader::to_json;

pub fn main() -> Result<()> {
    let matches = App::new("k2node to json")
        .author("strvert <strv@strv.dev>")
        .arg(
            Arg::with_name("input")
                .long("input")
                .short("i")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(false)
                .default_value("output.json"),
        )
        .arg(
            Arg::with_name("pretty")
                .long("pretty")
                .short("p")
                .takes_value(false),
        )
        .get_matches();

    let in_file = matches.value_of("input").unwrap();
    let out_file = matches.value_of("output").unwrap();
    let pretty = matches.is_present("pretty");

    let graph_code = match fs::read_to_string(in_file) {
        Ok(code) => code,
        Err(err) => panic!("Failed to open the file : {:?}", err),
    };

    let j = to_json(&graph_code, pretty)?;
    fs::write(out_file, j)?;

    Ok(())
}
