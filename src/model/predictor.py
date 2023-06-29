import datetime as dt

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import yfinance as yf
from keras.layers import Dense, Dropout, LSTM
from keras.models import Sequential
from sklearn.preprocessing import MinMaxScaler


def predict(symbol, start, end, period='max', testing_days=182, future_days=1):
    """
    Predicts the future stock prices of a given symbol.
    :param symbol: The ticker symbol of the stock.
    :param start: The start date of the training data.
    :param end: The end date of the training data.
    :param period: The period of the training data.
    :param testing_days: The number of days to test the model on.
    :param future_days: The number of days to predict.
    :return: The predictions and the accuracy of the model.
    """
    value = 'Close'
    
    data = yf.Ticker(symbol)
    data = data.history(period=period, start=start, end=end)
    
    # Prepare the data.
    scaler = MinMaxScaler(feature_range=(0, 1))
    scaled_data = scaler.fit_transform(data[value].values.reshape(-1, 1))

    x_train = []
    y_train = []
    
    for x in range(testing_days, len(scaled_data)):
        x_train.append(scaled_data[x - testing_days:x, 0])
        y_train.append(scaled_data[x, 0])
    
    x_train, y_train = np.array(x_train), np.array(y_train)
    x_train = np.reshape(x_train, (x_train.shape[0], x_train.shape[1], 1))
    
    # Build the LSTM model.
    epochs = 25
    units = 50
    batch_size = 32
    verbose = 1
    
    model = Sequential()
    
    model.add(LSTM(units=units, return_sequences=True, input_shape=(x_train.shape[1], 1)))
    model.add(Dropout(0.2))
    
    model.add(LSTM(units=units, return_sequences=True))
    model.add(Dropout(0.2))
    
    model.add(LSTM(units=units))
    model.add(Dropout(0.2))
    
    model.add(Dense(units=1))  # Prediction of the next closing value
    
    model.compile(optimizer='adam', loss='mean_squared_error')
    model.fit(x_train, y_train, epochs=epochs, batch_size=batch_size, verbose=verbose)
    
    # Test the model accuracy on existing data.
    # Load the data.
    test_period = period
    test_start = end
    test_end = dt.datetime.now().strftime('%Y-%m-%d')

    test_data = yf.Ticker(symbol)
    test_data = test_data.history(period=test_period, start=test_start, end=test_end)
    actual_prices = test_data[value].values
    
    total_dataset = pd.concat((data[value], test_data[value]), axis=0)
    
    model_inputs = total_dataset[len(total_dataset) - len(test_data) - testing_days:].values
    model_inputs = model_inputs.reshape(-1, 1)
    model_inputs = scaler.transform(model_inputs)
    
    # Make predictions on testing data.
    x_test = []
    
    for x in range(testing_days, len(model_inputs)):
        x_test.append(model_inputs[x - testing_days:x, 0])

    x_test = np.array(x_test)
    x_test = np.reshape(x_test, (x_test.shape[0], x_test.shape[1], 1))

    predicted_prices = model.predict(x_test)
    predicted_prices = scaler.inverse_transform(predicted_prices)

    accuracy = calculate_increase(actual_prices[-1], predicted_prices[-1])

    # Predict the next day.
    real_data = [model_inputs[len(model_inputs) + 1 - testing_days:len(model_inputs + 1), 0]]
    real_data = np.array(real_data)
    real_data = np.reshape(real_data, (real_data.shape[0], real_data.shape[1], 1))
    
    prediction = model.predict(real_data)
    prediction = scaler.inverse_transform(prediction)

    # Plot the testing predictions.
    plt.plot(actual_prices, color='black', label=f'Actual {symbol} Price')
    plt.plot(predicted_prices, color='green', label=f'Predicted {symbol} Price')
    plt.title(f'{symbol} Share Price')
    plt.xlabel('Time')
    plt.ylabel(f'{symbol} Share Price')
    plt.legend()
    plt.show()

    predictions = []
    for x in range(future_days):
        predictions.append(prediction[0][0])

        real_data = [np.append(real_data[0][1:], prediction[0])]
        real_data = np.array(real_data)
        real_data = np.reshape(real_data, (real_data.shape[0], real_data.shape[1], 1))

        prediction = model.predict(real_data)
        prediction = scaler.inverse_transform(prediction)

    increase = calculate_increase(actual_prices[-1], predictions[-1])

    # Return the predictions, increase, and accuracy.
    return predictions, increase, accuracy


def calculate_increase(start_price, end_price):
    """
    Calculates the increase/decrease in price from the start price to the end price.
    :param start_price: The starting price.
    :param end_price: The ending price.
    :return: The increase/decrease in price as a percentage.
    """
    return (end_price - start_price) / start_price * 100


if __name__ == '__main__':
    pred, inc, acc = predict('AAPL', '2012-01-01', '2021-01-01')

    print(f'Predictions: {pred}')
    print(f'Increase: {inc}%')
    print(f'Accuracy: {acc}%')
