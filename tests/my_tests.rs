use int6_miniproject8::{calc_stats, read_data};
use polars::prelude::*;

// Test for read_data function
fn test_read_data() {
    let data = read_data("data/spotify.csv").expect("Failed to read data");

    // Assert that the result is a DataFrame and not empty
    assert!(data.height() > 0, "Error: DataFrame is empty");
}

// Test for calc_stats function
fn test_calc_stats() -> Result<(), Box<dyn std::error::Error>> {
    // Create test data as a Polars DataFrame
    let df = DataFrame::new(vec![
        Series::new("popularity", &[5.0, 10.0, 15.0, 20.0]),
        Series::new("duration_ms", &[120000.0, 240000.0, 360000.0, 480000.0]),
        Series::new("danceability", &[0.5, 0.6, 0.7, 0.8]),
        Series::new("energy", &[0.3, 0.6, 0.9, 0.6]),
        Series::new("key", &[1.0, 2.0, 3.0, 4.0]),
        Series::new("loudness", &[-5.0, -6.0, -7.0, -8.0]),
        Series::new("mode", &[1.0, 0.0, 1.0, 0.0]),
        Series::new("speechiness", &[0.04, 0.05, 0.06, 0.07]),
        Series::new("acousticness", &[0.1, 0.2, 0.3, 0.4]),
        Series::new("instrumentalness", &[0.0, 0.1, 0.2, 0.3]),
        Series::new("liveness", &[0.2, 0.3, 0.4, 0.5]),
        Series::new("valence", &[0.3, 0.4, 0.5, 0.6]),
        Series::new("tempo", &[100.0, 110.0, 120.0, 130.0]),
        Series::new("time_signature", &[4.0, 4.0, 3.0, 3.0]),
    ])
    .expect("Failed to create test DataFrame");

    // Call calc_stats with the test DataFrame
    let stats_df = calc_stats(&df)?;

    // Extract the mean and median values for exact comparisons
    let mean_popularity = stats_df.column("popularity_mean")?.f64()?.get(0).unwrap();
    let median_duration_ms = stats_df
        .column("duration_ms_median")?
        .f64()?
        .get(0)
        .unwrap();
    let mean_key = stats_df.column("key_mean")?.f64()?.get(0).unwrap();
    let median_loudness = stats_df.column("loudness_median")?.f64()?.get(0).unwrap();

    // Assertions without using abs
    assert_eq!(mean_popularity, 12.50, "Mean popularity does not match");
    assert_eq!(
        median_duration_ms, 300000.0,
        "Median duration_ms does not match"
    );
    assert_eq!(mean_key, 2.50, "Mean key does not match");
    assert_eq!(median_loudness, -6.50, "Median loudness does not match");

    println!("All assertions passed for calc_stats.");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_read_data();
    test_calc_stats()?;
    Ok(())
}
