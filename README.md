# Stock Predictor

Predict the price of a stock using the power of machine learning!

# Table of Contents

* [Installation (Server Side)](#installation-server-side)
* [Usage (Client Side)](#usage-client-side)
* [Example Request](#example-request)
* [Example Response](#example-response)

# Installation (Server Side)

1. Clone the repository.
    ```sh
    git clone https://github.com/BastianAsmussen/Stock-Predictor.git as stock_predictor
    ```
2. Change directory to the project folder.
    ```sh
    cd stock_predictor
    ```
3. Build the Docker image.
    ```sh
    docker build -t stock_predictor .
    ```
4. Create a `.env` file.
    ```sh
    touch .env

    # Add the following to the .env file.
    # WORKERS=4
    # IP="127.0.0.1"
    # PORT=8080
    ```
5. Customize the docker-compose.yml file to your liking and then run it.
    ```sh
    docker-compose up --env-file .env
    ```

# Usage (Client Side)

1. Send a `GET` request to the server.
    ```sh
    curl -X GET "http://127.0.0.1:8080/predict" -H "Content-Type: application/json" -d '{ "symbol": "AAPL", "time": { "unit": "weeks", "value": 1 }, "datasetSize": { "unit": "months", "value": 6 } }'
    ```

# Example Request

```json
{
  "symbol": "AAPL",
  "time": {
    "unit": "weeks",
    "value": 1
  },
  "datasetSize": {
    "unit": "months",
    "value": 6
  }
}
```

## Request Fields

| Field       | Type   | Description                             |
|-------------|--------|-----------------------------------------|
| symbol      | string | The stock symbol.                       |
| time        | object | The time unit and value to use.         |
| datasetSize | object | The dataset size unit and value to use. |

## Time Unit Fields

| Unit   | Value | Description                  |
|--------|-------|------------------------------|
| days   | int   | The number of days to use.   |
| weeks  | int   | The number of weeks to use.  |
| months | int   | The number of months to use. |
| years  | int   | The number of years to use.  |

# Example Response

```json
{
  "request": {
    "symbol": "AAPL",
    "time": {
      "unit": "weeks",
      "value": 1
    },
    "datasetSize": {
      "unit": "months",
      "value": 6
    }
  },
  "errorMessage": null,
  "currentAdjustedClose": 188.05999755859375,
  "modelR2Score": 0.9961243083624973,
  "predictions": [
    185.89104281924475,
    185.89207717957356,
    185.893102549463,
    185.89411900705608,
    185.89512662981653,
    185.89612549453486,
    185.8971156773341
  ],
  "increase": -0.011501020468671167,
  "shouldBuy": false
}
```

## Response Fields

| Field                | Type    | Description                                       |
|----------------------|---------|---------------------------------------------------|
| request              | object  | The request that was sent to the server.          |
| errorMessage         | string  | The error message if an error occurred.           |
| currentAdjustedClose | float   | The current adjusted close price of the stock.    |
| modelR2Score         | float   | The R2 score of the model.                        |
| predictions          | float[] | The predicted adjusted close prices of the stock. |
| increase             | float   | The increase or decrease of the stock.            |
| shouldBuy            | boolean | Whether or not the stock should be bought.        |
