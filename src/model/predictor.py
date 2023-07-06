import numpy as np
import tensorflow as tf
import yfinance as yf
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import MinMaxScaler


def predict(ticker, future_days):
    # Download historical stock data using yfinance
    data = yf.download(ticker)

    # Preprocess the data
    data = data[["Close"]]  # Selecting only the 'Close' price column
    data["Close"] = data["Close"].ffill()  # Forward fill any missing values

    # Normalize the data
    scaler = MinMaxScaler(feature_range=(0, 1))
    data["Close"] = scaler.fit_transform(data["Close"].values.reshape(-1, 1))

    # Create input features and target variable
    n_steps = 10  # Number of time steps to consider
    x, y = [], []
    for i in range(n_steps, len(data)):
        x.append(data.iloc[i - n_steps: i, 0].values)
        y.append(data.iloc[i, 0])
    x, y = np.array(x), np.array(y)

    # Split the data into training and testing sets
    x_train, x_test, y_train, y_test = train_test_split(
        x, y, test_size=0.2, shuffle=False
    )

    # Build the neural network model
    model = tf.keras.Sequential(
        [
            tf.keras.layers.Dense(64, activation="relu", input_shape=(n_steps,)),
            tf.keras.layers.Dense(64, activation="relu"),
            tf.keras.layers.Dense(1),
        ]
    )

    # Compile the model
    model.compile(optimizer="adam", loss="mean_squared_error")

    # Train the model
    model.fit(x_train, y_train, epochs=50, batch_size=32, verbose=0)

    # Predict on the next n days
    predictions = []
    # Use the last n days from today (normalized) as input features
    last_n_days = data["Close"].values[-n_steps:].reshape(1, -1)
    for i in range(future_days):
        pred = model.predict(last_n_days)
        scaled_pred = scaler.inverse_transform(pred)
        predictions.append(scaled_pred[0][0])
        last_n_days = np.append(last_n_days[:, 1:], pred[0]).reshape(1, -1)

    # Evaluate the model on the test set
    loss = model.evaluate(x_test, y_test)

    return predictions, loss
