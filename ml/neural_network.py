import os
import numpy as np
import json


class TestCase:
    @staticmethod
    def load(filename):
        with open(filename, "r") as f:
            objs = json.load(f)
        test_cases = []
        for obj in objs:
            test_cases.append(TestCase(obj["input"], obj["label"]))
        return test_cases

    def __init__(self, input, expected):
        self.input = np.array([float(x) for x in input]).reshape(-1, 1)
        self.expected = np.array(float(expected)).reshape(-1, 1)


# Neural network inspired by 3B1B
# https://www.youtube.com/playlist?list=PLZHQObOWTQDNU6R1_67000Dx_ZCJB-3pi


def sigmoid(x):
    """Sigmoid function"""
    return 1.0 / (1.0 + np.exp(-x))


def d_sigmoid(x):
    """Derivative of the sigmoid function"""
    s = sigmoid(x)
    return s * (1 - s)


class NeuralNetwork:
    def __init__(self, shape, seed=1):
        n = shape
        L = len(n)
        W = []
        B = []

        np.random.seed(seed)
        for l in range(1, L):
            W.append(np.random.randn(n[l], n[l - 1]))
            B.append(np.random.randn(n[l], 1))

        self.n, self.L, self.W, self.B = n, L, W, B

    def run(self, input):
        """
        Runs the neural network using current weights
        `input` - column vector of length n[0]
        """
        n, L, W, B = self.n, self.L, self.W, self.B
        assert input.shape == (n[0], 1)
        A = [input]

        for l in range(1, L):
            a = sigmoid(W[l - 1] @ A[l - 1] + B[l - 1])
            A.append(a)
        return A[L - 1]

    def run_cost(self, input, expected):
        """
        Returns the cost of the neural network on a given input
        `input` - column vector of length n[0]
        `expected` - column vector of length n[L - 1]
        """
        assert expected.shape == (self.n[self.L - 1], 1)
        A_L = self.run(input)
        Y = expected

        C = np.square(A_L - Y)
        c = np.sum(C)
        return c

    def run_cost_avg(self, batch):
        """
        Returns the average cost of a batch of inputs
        """
        total = 0
        for case in batch:
            total += self.run_cost(case.input, case.expected)
        return total / len(batch)

    def backprop(self, input, expected):
        """
        Execute one round of the backpropigation algorithm
        with the given `input` and `expected` vectors.
        """
        n, L, W, B = self.n, self.L, self.W, self.B
        assert input.shape == (n[0], 1)
        assert expected.shape == (n[L - 1], 1)
        A = [None] * L
        A[0] = input
        Z = [None] * (L - 1)
        Y = expected

        dA = [None] * L
        dZ = [None] * (L - 1)
        dB = [None] * (L - 1)
        dW = [None] * (L - 1)

        # Run forward
        for l in range(1, L):
            Z[l - 1] = W[l - 1] @ A[l - 1] + B[l - 1]
            A[l] = sigmoid(Z[l - 1])

        # Backprop
        for l in reversed(range(1, L)):
            dA[l] = 2 * (A[l] - Y) if l == L - 1 else W[l].T @ dZ[l]
            dZ[l - 1] = dA[l] * d_sigmoid(Z[l - 1])
            dB[l - 1] = dZ[l - 1]
            dW[l - 1] = dZ[l - 1] @ A[l - 1].T
        return (dW, dB)

    def train_batch(self, batch, eta=3.0):
        """
        Train a batch of test cases using backprop algorithm
        """
        dW_avg = [np.zeros(w.shape) for w in self.W]
        dB_avg = [np.zeros(b.shape) for b in self.B]
        for case in batch:
            dW, dB = self.backprop(case.input, case.expected)
            dW_avg = [w + dW for w, dW in zip(dW, dW_avg)]
            dB_avg = [b + dB for b, dB in zip(dB, dB_avg)]
        self.W = [w - (dw * eta / len(batch)) for w, dw in zip(self.W, dW_avg)]
        self.B = [b - (db * eta / len(batch)) for b, db in zip(self.B, dB_avg)]

    def train(self, train_set, seed, batch_size=100):
        """
        Train the model with an array of test cases
        """
        train_set = list(train_set)
        np.random.seed(seed)
        np.random.shuffle(train_set)

        batches = [
            train_set[i : i + batch_size] for i in range(0, len(train_set), batch_size)
        ]

        for batch in batches:
            self.train_batch(batch)

    @staticmethod
    def load(filename):
        try:
            with open(filename, "r") as f:
                obj = json.load(f)
        except FileNotFoundError:
            return None
        net = NeuralNetwork(tuple(obj["shape"]))
        net.W = [np.array(w) for w in obj["weights"]]
        net.B = [np.array(b) for b in obj["biases"]]
        return net

    def save(self, filename):
        # os.makedirs(os.path.dirname(filename), exist_ok=True)
        obj = {
            "shape": self.n,
            "weights": [w.tolist() for w in self.W],
            "biases": [b.tolist() for b in self.B],
        }
        with open(filename, "w") as f:
            json.dump(obj, f)
