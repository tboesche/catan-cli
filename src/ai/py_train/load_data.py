import os
import pandas as pd
import torch
from torch.utils.data import Dataset, DataLoader
import torch.nn as nn
import torch.nn.functional as F
import torch.optim as optim

class GamesData(Dataset):
    def __init__(self, csv_dir, labels_file, transform=None):
        """
        Args:
            csv_dir (string): Directory with all the CSV files.
            labels_file (string): Path to the labels CSV file.
            transform (callable, optional): Optional transform to be applied on a sample.
        """
        self.csv_dir = csv_dir
        self.labels_df = pd.read_csv(labels_file)
        self.transform = transform

    def __len__(self):
        return len(self.labels_df)

    def __getitem__(self, idx):
        if torch.is_tensor(idx):
            idx = idx.tolist()
        
        # Get the file name and label
        file_name = self.labels_df.iloc[idx, 0]
        label = self.labels_df.iloc[idx, 1]
        
        # Load the data from the CSV file
        file_path = os.path.join(self.csv_dir, file_name + '.csv')
        data = pd.read_csv(file_path, header=None)
        
        # Convert to tensor
        data = torch.tensor(data.values, dtype=torch.float32)
        label = torch.tensor(label, dtype=torch.float32)
        
        sample = {'data': data, 'label': label}
        
        if self.transform:
            sample = self.transform(sample)
        
        return sample
    

# Directory containing the CSV files
csv_dir = 'data/ai/random_ne_processed/individual_files'

# Path to the labels CSV file
labels_file = 'data/ai/random_ne_processed/labels.csv'

# Create the dataset
csv_dataset = GamesData(csv_dir=csv_dir, labels_file=labels_file)

# Create the DataLoader
dataloader = DataLoader(csv_dataset, batch_size=256, shuffle=True, num_workers=8)

# Example of iterating through the DataLoader
for batch in dataloader:
    print(batch['data'])
    print(batch['label'])