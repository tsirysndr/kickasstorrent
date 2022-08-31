use colored_json::ToColoredJson;
use tabled::{Style, Table};

use crate::types::Torrent;

pub fn format_results(results: Vec<Torrent>, json: bool) {
    if json {
        println!(
            "{}",
            serde_json::to_string(&results)
                .unwrap()
                .to_colored_json_auto()
                .unwrap()
        );
        return;
    };
    println!("{}", Table::new(&results).with(Style::psql()));
}
