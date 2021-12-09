use anyhow::Result;
use bp_graph_parser::parse_graph;
use clap::{App, Arg};
use std::fs;

pub fn main() -> Result<()> {
    let matches = App::new("k2node to json")
        .author("strvert <strv@strv.dev>")
        .arg(
            Arg::with_name("input")
                .long("input")
                .short("i")
                .takes_value(true),
        )
        .arg(Arg::with_name("output").long("output").short("o"))
        .get_matches();

    let in_file = matches.value_of("input").unwrap();

    let graph_code = match fs::read_to_string(in_file) {
        Ok(code) => code,
        Err(err) => panic!("ファイルのオープンに失敗しました: {:?}", err),
    };

    let tree = parse_graph(&graph_code);
    println!("{:#?}", tree);

    Ok(())
}
