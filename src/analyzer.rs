use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Main function to analyze the log file
pub fn analyze_log_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the log file and create a buffered reader for efficient reading
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Define the regex pattern to capture relevant log data (IP, timestamp, HTTP method, path, status code)
    let log_pattern = Regex::new(
        r#"(?P<ip>\d+\.\d+\.\d+\.\d+).*\[(?P<time>.*?)\] "(?P<method>\w+) (?P<path>\S+) .*" (?P<status>\d{3})"#,
    )?;

    // HashMaps to store the counts for URLs, status codes, and IP addresses
    let mut url_count = HashMap::new();
    let mut status_count = HashMap::new();
    let mut ip_count = HashMap::new();

    // Iterate through each line in the log file
    for line in reader.lines() {
        let line = line?; // Read each line
        // Apply regex to capture the relevant fields (IP, path, status)
        if let Some(caps) = log_pattern.captures(&line) {
            let path = caps["path"].to_string(); // Extract the URL path
            let ip = caps["ip"].to_string(); // Extract the IP address
            let status = caps["status"].to_string(); // Extract the HTTP status code

            // Update the counts for URLs, status codes, and IPs
            *url_count.entry(path).or_insert(0) += 1;
            *status_count.entry(status).or_insert(0) += 1;
            *ip_count.entry(ip).or_insert(0) += 1;
        }
    }

    // Print the top 5 most requested URLs
    println!("\n Top 5 Requested URLs:");
    print_top(&url_count, 5);

    // Print the distribution of status codes
    println!("\n Status Code Distribution:");
    print_top(&status_count, status_count.len());

    // Print the top 3 visitor IPs
    println!("\n Top 3 Visitor IPs:");
    print_top(&ip_count, 3);

    Ok(())
}

// Helper function to print the top N items from a HashMap
fn print_top(map: &HashMap<String, usize>, top_n: usize) {
    // Convert the HashMap into a vector of tuples and sort by count in descending order
    let mut items: Vec<_> = map.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));

    // Print the top N items
    for (i, (key, count)) in items.into_iter().take(top_n).enumerate() {
        println!("{}. {} - {} requests", i + 1, key, count);
    }
}
