use polars::prelude::*;
use std::error::Error;
use std::time::Instant;

// Function to read data from CSV into a Polars DataFrame
pub fn read_data(path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let df = CsvReader::from_path(path)?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()?;
    Ok(df)
}

// Function to calculate mean, median, and standard deviation for specified columns
pub fn calc_stats(df: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let mut stats = Vec::new();

    for col in df.get_columns() {
        if col.dtype() == &DataType::Float64 || col.dtype() == &DataType::Int64 {
            let mean = col.mean().unwrap_or(f64::NAN);
            let median = col.median().unwrap_or(f64::NAN);

            stats.push(Series::new(&format!("{}_mean", col.name()), &[mean]));
            stats.push(Series::new(&format!("{}_median", col.name()), &[median]));
        }
    }

    let stats_df = DataFrame::new(stats)?;
    Ok(stats_df)
}

pub fn performance() -> Result<(), Box<dyn Error>> {
    // Load the full dataset once
    let df = read_data("data/spotify.csv")?;
    let initial_memory = sys_info::mem_info()?.total;
    let start_time = Instant::now();
    let _stats = calc_stats(&df)?;

    let duration = start_time.elapsed();
    let final_memory = sys_info::mem_info()?.total;
    let used_memory = initial_memory.saturating_sub(final_memory);

    println!("Time taken: {:.6} seconds", duration.as_secs_f64());
    println!("Memory used: {} KB", used_memory);

    Ok(())
}
