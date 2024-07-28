import os
import pandas as pd
import torch
from torch.utils.data import Dataset, DataLoader, random_split
import torch.nn as nn
import torch.nn.functional as F
import torch.optim as optim
import h5py

class GamesData(Dataset):
    def __init__(self, file_path):
        self.file_path = file_path

    def __len__(self):
        with h5py.File(self.file_path, 'r') as hf:
            return len(hf['names'])
    
    def __getitem__(self, idx):
        with h5py.File(self.file_path, 'r') as hf:
            label = torch.tensor(hf['labels'][idx], dtype=torch.float32)
            features = torch.tensor(hf['features'][idx], dtype=torch.float32)
        return features, label

class EarlyStopping:
    def __init__(self, patience=5, min_delta=0):
        self.patience = patience
        self.min_delta = min_delta
        self.counter = 0
        self.best_loss = None
        self.early_stop = False

    def __call__(self, val_loss):
        if self.best_loss is None:
            self.best_loss = val_loss
        elif val_loss > self.best_loss - self.min_delta:
            self.counter += 1
            if self.counter >= self.patience:
                self.early_stop = True
        else:
            self.best_loss = val_loss
            self.counter = 0

class SimpleNN(nn.Module):
    def __init__(self, input_size, num_classes):
        super(SimpleNN, self).__init__()
        self.layer1 = nn.Linear(input_size, 1024)
        self.layer2 = nn.Linear(1024, 512)
        self.layer3 = nn.Linear(512, 256)
        self.layer4 = nn.Linear(256, 128)
        self.layer5 = nn.Linear(128, 64)
        self.layer6 = nn.Linear(64, 32)
        self.output_layer = nn.Linear(32, num_classes)
    
    def forward(self, x):
        x = F.relu(self.layer1(x))
        x = F.relu(self.layer2(x))
        x = F.relu(self.layer3(x))
        x = F.relu(self.layer4(x))
        x = F.relu(self.layer5(x))
        x = F.relu(self.layer6(x))
        x = self.output_layer(x)
        return x.view(-1, num_classes)

if torch.cuda.is_available():
    device = torch.device("cuda")
    print("Using GPU:", torch.cuda.get_device_name(0))
else:
    device = torch.device("cpu")
    print("CUDA is not available. Using CPU.")

input_size = 2368
num_classes = 1

model = SimpleNN(input_size=input_size, num_classes=num_classes)
model.to(device)

criterion = nn.MSELoss()
optimizer = optim.Adam(model.parameters(), lr=0.001)

source_dir = 'data/ai/random_ne_processed/data.hdf5'

csv_dataset = GamesData(source_dir)

# Split the dataset into train and validation sets
train_size = int(0.8 * len(csv_dataset))
val_size = len(csv_dataset) - train_size
train_dataset, val_dataset = random_split(csv_dataset, [train_size, val_size])

train_loader = DataLoader(train_dataset, batch_size=2048, shuffle=True, num_workers=16)
val_loader = DataLoader(val_dataset, batch_size=2048, shuffle=False, num_workers=16)

num_epochs = 100
patience = 5
min_delta = 0.001
early_stopping = EarlyStopping(patience=patience, min_delta=min_delta)

# # Custom learning rate scheduler
# def custom_lr_scheduler(epoch, num_epochs, initial_lr=0.001, max_lr=1.0):
#     if epoch < num_epochs // 4:
#         return initial_lr + (max_lr - initial_lr) * (epoch / (num_epochs // 4))
#     elif epoch < 3 * num_epochs // 8:
#         return max_lr
#     else:
#         return max_lr - (max_lr - initial_lr) * ((epoch - 3 * num_epochs // 8) / (5 * num_epochs // 8))

for epoch in range(num_epochs):
    # # Update learning rate
    # lr = custom_lr_scheduler(epoch, num_epochs)
    # for param_group in optimizer.param_groups:
    #     param_group['lr'] = lr
    
    model.train()
    running_loss = 0.0
    for batch in train_loader:
        data = batch[0].to(device)
        labels = batch[1].to(device)

        optimizer.zero_grad()

        outputs = model(data)
        loss = criterion(outputs, labels.unsqueeze(1).float())

        loss.backward()
        optimizer.step()

        running_loss += loss.item()
    
    avg_train_loss = running_loss / len(train_loader)
    
    model.eval()
    val_loss = 0.0
    with torch.no_grad():
        for batch in val_loader:
            data = batch[0].to(device)
            labels = batch[1].to(device)
            
            outputs = model(data)
            loss = criterion(outputs, labels.unsqueeze(1).float())
            val_loss += loss.item()
    
    avg_val_loss = val_loss / len(val_loader)
    
    print(f'Epoch [{epoch+1}/{num_epochs}], Train Loss: {avg_train_loss:.4f}, Val Loss: {avg_val_loss:.4f}')
    
    early_stopping(avg_val_loss)
    if early_stopping.early_stop:
        print("Early stopping")
        break

# Save the model
torch.save(model.state_dict(), 'srmodel.pth')
print("Model saved as model.pth")
