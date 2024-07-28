import os
import pandas as pd
import polars as pl

path_in = "data/saves/test_random_ne/encoded_games.csv"

path_out = "data/ai/random_ne"
os.makedirs(path_out, exist_ok=True)

# Define chunk size
chunk_size = 10**5  # Adjust this according to your memory capacity


# Read the CSV file in chunks
chunk_iter = pd.read_csv(path_in, chunksize=chunk_size)

# Process each chunk
for chunk in chunk_iter:
    # Group the chunk by 'game_id'
    grouped = chunk.groupby('game_id')
    
    # Iterate over each group
    for game_id, group in grouped:
        # Define the file name based on the game_id
        file_name = os.path.join(path_out, f"{game_id}.csv")
        
        # Append to the CSV file if it exists, otherwise create a new one
        if os.path.exists(file_name):
            group.to_csv(file_name, mode='a', header=False, index=False)
        else:
            group.to_csv(file_name, mode='w', header=True, index=False)

        print(f"Processed and saved data for game_id: {game_id}")
