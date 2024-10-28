import pandas as pd
import time
import sys


def read_data():
    data = pd.read_csv("data/spotify.csv")
    return data


def calc_stats(df):
    # select numerical columns of interest
    sub_df = df[
        [
            "popularity",
            "duration_ms",
            "danceability",
            "energy",
            "key",
            "loudness",
            "mode",
            "speechiness",
            "acousticness",
            "instrumentalness",
            "liveness",
            "valence",
            "tempo",
            "time_signature",
        ]
    ]
    # calc summary stats
    stats = {"mean": sub_df.mean(), "median": sub_df.median()}
    # return as df
    stats_df = pd.DataFrame(stats).round(2)
    return stats_df


def performance():
    results = []

    # Generate the dataset
    df = read_data()

    # Measure runtime
    start_time = time.time()
    calc_stats(df)
    end_time = time.time()
    duration = end_time - start_time

    # Measure memory usage
    memory = sys.getsizeof(df)

    # Print results
    print(f"Time usage: {duration:.10f} seconds")
    print(f"Memory usage: {memory} bytes")
    print("-" * 40)
    results.append((duration, memory))
    return results


if __name__ == "__main__":
    spotify_df = read_data()
    print(calc_stats(spotify_df))
    print(performance())
