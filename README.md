# CS128HonorProject
# Group Name
Rust Log Analyzer 

# Group Members
- Zhen Yu (NetID: zheny5)
- Carol Yin (NetID：carolry2)
---

## Project Introduction

We will develop a **Rust-based Log Analyzer**, a Command-Line Tool designed to help developers and system administrators analyze server logs (e.g., Nginx, Apache, or custom application logs) efficiently. 

The goal is to extract valuable information from massive log files, such as:
- Most frequently visited URLs
- Error distribution (404, 500, etc.)
- Request response time statistics
- Top IP addresses
- Access trends over time (hourly/daily)

We chose this project because it combines Rust's strengths (performance, safe concurrency) with practical log analysis. It is both technically challenging and useful in real-world scenarios.

---

## Technical Overview

The project will consist of the following core components:

1. **Log File Reader**
   - Support for reading `.log` / `.txt` files.
   - CLI interface to specify the log file path.
   - Stream large files line-by-line efficiently.

2. **Log Parser**
   - Parse logs using `regex`.
   - Extract important fields: IP, timestamp, HTTP method, path, status code, response time.

3. **Data Analyzer**
   - Calculate statistics:
     - Most accessed URLs
     - Status code distribution
     - Average response time
     - Top IP addresses
   - Use `polars` for data processing.
   - Use `rayon` for parallel processing to speed up analysis of large logs.

4. **CLI Output**
   - Present results in a clear and user-friendly format.
   - Display statistics directly in the terminal.

5. *(Optional)* **Visualization**
   - Use `plotly-rs` to generate charts.
   - Export graphs as HTML or PNG.

---

### Checkpoints

| Date | Goal |
|------|------|
| **Checkpoint 1** | Implement log file reading, parsing, and basic statistics (URL counts, status code distribution, IP ranking). |
| **Checkpoint 2** |Add multi-threading with rayon for performance improvements, refine CLI output formatting, and add optional visualization (charts).|

---

## Possible Challenges

- Learning to use `polars` and `rayon` effectively.
- Handling very large log files efficiently.
- Designing a clean and user-friendly CLI interface.
- (Optional) Learning `plotly-rs` for visualization.

---

## References

- [Nginx Access Log Format](https://nginx.org/en/docs/http/ngx_http_log_module.html)
- [`regex` crate](https://docs.rs/regex/latest/regex/)
- [`polars` crate](https://docs.rs/polars/latest/polars/)
- [`rayon` crate](https://docs.rs/rayon/latest/rayon/)


