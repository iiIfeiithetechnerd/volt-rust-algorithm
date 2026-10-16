import numpy as np

class neural_network:
    def __init__(self, input_size, hidden_size, output_size, learning_rate = 0.1):
        self.input_size = input_size
        self.hidden_size = hidden_size
        self.output_size = output_size
        self.learning_rate = learning_rate
        
        # Initialize weights and biases
        self.weights_input_hidden = np.random.rand(self.input_size, self.hidden_size)
        self.weights_hidden_output = np.random.rand(self.hidden_size, self.output_size)
        self.bias_hidden = np.random.rand(self.hidden_size)
        self.bias_output = np.random.rand(self.output_size)

    def relu(self, Z):
        return np.maximum(0, Z)

    def relu_derivative(self, Z):
        return (Z > 0).astype(float)

    def forward(self, X):
        self.X = X

        self.Z1 = np.dot(X, self.weights_input_hidden) + self.bias_hidden
        self.A1 = self.relu(self.Z1)

        self.Z2 = np.dot(self.A1, self.W2) + self.b2
        self.A2 = self.sigmoid(self.Z2)
        return self.A2

    def backward(self, y):
        m = y.shape[0]

        # Calculate the error
        self.error_output = self.A2 - y
        self.error_hidden = np.dot(self.error_output, self.W2.T) * self.sigmoid_derivative(self.Z1)

        # Update weights and biases
        self.W2 -= (self.learning_rate / m) * np.dot(self.A1.T, self.error_output)
        self.b2 -= (self.learning_rate / m) * np.sum(self.error_output, axis=0)

        self.W1 -= (self.learning_rate / m) * np.dot(self.X.T, self.error_hidden)
        self.b1 -= (self.learning_rate / m) * np.sum(self.error_hidden, axis=0)
        