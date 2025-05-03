use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn analyze_log_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let log_pattern = Regex::new(
        r#"(?P<ip>\d+\.\d+\.\d+\.\d+).*\[(?P<time>.*?)\] "(?P<method>\w+) (?P<path>\S+) .*" (?P<status>\d{3})"#,
    )?;

    let mut url_count = HashMap::new();
    let mut status_count = HashMap::new();
    let mut ip_count = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = log_pattern.captures(&line) {
            let path = caps["path"].to_string();
            let ip = caps["ip"].to_string();
            let status = caps["status"].to_string();

            *url_count.entry(path).or_insert(0) += 1;
            *status_count.entry(status).or_insert(0) += 1;
            *ip_count.entry(ip).or_insert(0) += 1;
        }
    }

    println!("\n📈 Top 5 Requested URLs:");
    print_top(&url_count, 5);

    println!("\n🚦 Status Code Distribution:");
    print_top(&status_count, status_count.len());

    println!("\n🌐 Top 3 Visitor IPs:");
    print_top(&ip_count, 3);

    Ok(())
}

fn print_top(map: &HashMap<String, usize>, top_n: usize) {
    let mut items: Vec<_> = map.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));
    for (i, (key, count)) in items.into_iter().take(top_n).enumerate() {
        println!("{}. {} - {} requests", i + 1, key, count);
    }
}
