import tensorflow as tf
import numpy as np
from tensorflow.keras import layers, models, datasets
from colorama import Fore, Style

# Load CIFAR-10 dataset
(x_train, y_train), (x_test, y_test) = datasets.cifar10.load_data()
x_train, x_test = x_train / 255.0, x_test / 255.0

# Define model architecture
def create_model():
    model = models.Sequential([
        layers.Conv2D(32, (3, 3), activation='relu', input_shape=(32, 32, 3)),
        layers.MaxPooling2D((2, 2)),
        layers.Conv2D(64, (3, 3), activation='relu'),
        layers.MaxPooling2D((2, 2)),
        layers.Conv2D(64, (3, 3), activation='relu'),
        layers.Flatten(),
        layers.Dense(64, activation='relu'),
        layers.Dense(10)
    ])
    return model

# Define number of rounds and clients
NUM_ROUNDS = 10
NUM_CLIENTS = 2

# Initialize optimizer
optimizer = tf.keras.optimizers.SGD(learning_rate=0.01)

# Federated learning loop
for round_num in range(NUM_ROUNDS):
    print(f"{Fore.GREEN}Round {round_num+1}/{NUM_ROUNDS}{Style.RESET_ALL}")

    # Create server model
    server_model = create_model()

    # Initialize client models
    client_models = [create_model() for _ in range(NUM_CLIENTS)]

    # Clients train their models
    for i, client_model in enumerate(client_models):
        print(f"{Fore.RED}Client {i+1} training...{Style.RESET_ALL}")
        client_model.compile(optimizer='sgd', loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=True), metrics=['accuracy'])
        client_model.fit(x_train[i * len(x_train) // NUM_CLIENTS:(i + 1) * len(x_train) // NUM_CLIENTS],
                         y_train[i * len(y_train) // NUM_CLIENTS:(i + 1) * len(y_train) // NUM_CLIENTS],
                         epochs=1, batch_size=32, verbose=0)
        print(f"{Fore.BLUE}Client {i+1} training completed.{Style.RESET_ALL}")

    # Aggregate client models at the server
    for client_model in client_models:
        for server_layer, client_layer in zip(server_model.layers, client_model.layers):
            server_layer_weights = server_layer.get_weights()
            client_layer_weights = client_layer.get_weights()
            averaged_weights = [(sw + cw) / NUM_CLIENTS for sw, cw in zip(server_layer_weights, client_layer_weights)]
            server_layer.set_weights(averaged_weights)

    # Evaluate server model
    print(f"{Fore.LIGHTMAGENTA_EX}Server model evaluation:{Style.RESET_ALL}")
    server_model.compile(optimizer=optimizer, loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=True), metrics=['accuracy'])
    test_loss, test_acc = server_model.evaluate(x_test, y_test, verbose=2)
    print(f"{Fore.RED}Test accuracy: {test_acc}{Style.RESET_ALL}")
