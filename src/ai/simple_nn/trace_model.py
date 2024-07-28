import torch

import torch.nn as nn
import torch.nn.functional as F

from safetensors import torch as stt

num_classes = 1

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


model = SimpleNN(2368, 1)
model.load_state_dict(torch.load("src/ai/simple_nn/model.pth"))

# model = torch.load("src/ai/simple_nn/model.pth")

# print(model.weight)

example = torch.rand(1, 2368)

traced_model = torch.jit.trace(model, example)
# traced_model = torch.jit.script(model)

# print(traced_model)

stt.save_file(model.state_dict(), 'src/ai/simple_nn/weights.safetensors')

# traced_model.save("src/ai/simple_nn/traced_model.pt")