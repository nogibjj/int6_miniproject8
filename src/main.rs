use int6_miniproject8::{calc_stats, performance, read_data};
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the data
    let df = read_data("data/spotify.csv")?;
    let stats_df = calc_stats(&df)?;

    println!("{:20} {:>10} {:>10}", "Column", "Mean", "Median");
    println!("{:-<42}", ""); // Separator line

    // Iterate through pairs of columns (mean and median)
    let columns = stats_df.get_columns();
    for i in (0..columns.len()).step_by(2) {
        let mean_series = &columns[i];
        let median_series = &columns[i + 1];
        let column_name = mean_series.name().replace("_mean", "");

        // Get the mean and median values
        let mean_value = mean_series.get(0).unwrap_or(AnyValue::Null);
        let median_value = median_series.get(0).unwrap_or(AnyValue::Null);

        // Print each row with aligned formatting
        println!(
            "{:20} {:>10.2} {:>10.2}",
            column_name,
            mean_value.try_extract::<f64>().unwrap_or(f64::NAN),
            median_value.try_extract::<f64>().unwrap_or(f64::NAN),
        );
    }
    performance()?;
    Ok(())
}
