# Stock Predictor

Predict the price of a stock using the power of machine learning!

# Table of Contents

* [Installation (Server Side)](#installation-server-side)
* [Environment Variables (Server Side)](#environment-variables-server-side)
* [Usage (Client Side)](#usage-client-side)
* [Example Request](#example-request)
* [Example Response](#example-response)
* [Compiling](#compiling)

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

## Environment Variables (Server Side)

| Variable | Description                                                 |
|----------|-------------------------------------------------------------|
| WORKERS  | The number of workers (threads) to use for the HTTP server. |
| IP       | The IP address to use for the HTTP server.                  |
| PORT     | The port to use for the HTTP server.                        |

# Usage (Client Side)

1. Send a `GET` request to the server.
    ```sh
    curl -X GET "http://127.0.0.1:8080/predict" -H "Content-Type: application/json" -d '{ "symbol": "AAPL", "days": 7 }'
    ```

# Example Request

```json
{
  "symbol": "AAPL",
  "days": 7
}
```

## Request Fields

| Field  | Type   | Description                                    |
|--------|--------|------------------------------------------------|
| symbol | string | The stock symbol.                              |
| days   | int    | The number of days to predict into the future. |

# Example Response

```json
{
  "request": {
    "symbol": "AAPL",
    "days": 7
  },
  "error": null,
  "predictions": [
    191.48390197753906,
    192.28878784179688,
    192.40756225585938,
    192.34530639648438,
    192.67547607421875,
    193.71231079101562,
    193.5487518310547
  ],
  "modelRmse": 0.00010840992763405666
}
```

## Response Fields

| Field       | Type    | Description                               |
|-------------|---------|-------------------------------------------|
| request     | object  | The request that was sent to the server.  |
| error       | string  | The error message if an error occurred.   |
| predictions | array   | The predicted prices.                     |
| modelRmse   | float64 | The root mean squared error of the model. |

# Compiling

1. Clone the repository.
    ```sh
    git clone https://github.com/BastianAsmussen/Stock-Predictor.git as stock_predictor
    ```
2. Change directory to the project folder.
    ```sh
    cd stock_predictor
    ```
3. Compile the project.
    ```sh
    cargo build --release
    ```
4. Getting logger information. (Optional)
   ### Linux
   ```sh
   source RUST_LOG=stock_predictor # Set the environment variable.
    ```
   ### Windows
    ```sh
    set RUST_LOG=stock_predictor # Set the environment variable.
    ```
5. Run the project.
   ### Linux
    ```sh
    ./target/release/stock_predictor
    ```
   ### Windows
    ```sh
    .\target\release\stock_predictor.exe
    ```
