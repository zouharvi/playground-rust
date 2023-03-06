import torch

class Vocabulary:
    def __init__(self):
        self.mapper = {"UNK": 0, "BOS": 1, "EOS": 2}
        pass

    def fit(self, data):
        for word in data:
            if word not in self.mapper:
                self.mapper[word] = len(self.mapper)

    def transform(self, data):
        output = [
            self.mapper[word] if word in self.mapper else self.mapper["UNK"]
            for word in data
        ]
        return output
    
    def fit_transform(self, data):
        self.fit(data)
        return self.transform(data)

    def transform_onehot(self, sent):
        sent = self.transform(sent)
        sent = ["BOS"] + sent + ["EOS"]
        return torch.stack([
            torch.nn.functional.one_hot(torch.tensor(word), num_classes=len(self.mapper)).to(dtype=torch.long)
            for word in sent
        ])

data = [
    ("hello how are you ?", "Hallo, wie geht's Ihnen ?"),
    ("Hello there ! ! !", "Hallo da ! ! !"),
    ("There is a cat .", "Da ist eine Katze ."),
]
data = [(x.lower().split(), y.lower().split()) for x, y in data]
vocab = Vocabulary()
vocab.fit([word for y in data for sent in y for word in sent])
data_en = [vocab.transform_onehot(sent[0]) for sent in data]
data_de = [vocab.transform_onehot(sent[1]) for sent in data]

# all same length (5)
print([x.shape for x in data_en])
print([x.shape for x in data_de])


class TransformerBlock(torch.nn.Module):
    def __init__(self):
        pass

    def forward(self, x):
        pass

class Transformer(torch.nn.Module):
    def __init__(self, input_dim):
        self.embd = torch.nn.Linear(in_features=input_dim, out_features=64)
        self.blocks = [TransformerBlock()]
        self.loss = torch.nn.CrossEntropyLoss()
        self.optimizer = torch.optim.Adam(lr=10e-3)

    def forward(self, x, y):
        x = self.embd(x)
        # process input sentence
        for block in self.blocks:
            x = block(x)

        decoded = self.embd(vocab.transform_onehot(["BOS"]))
        for block in self.blocks:
            decoded = block(decoded)

        # use teacher forcing
        pass

    def train_cycle(self, data_train):
        self.train(True)
        for sent in data_train:
            while True:
                self.forward(sent, )
        pass

    def translate(self, sent):
        pass