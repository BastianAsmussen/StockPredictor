FROM rust:1.67

# Build the server binary.
WORKDIR /usr/src/stock_predictor
COPY . .
RUN cargo build --release

# Move the server binary to the root directory.
RUN mv ./target/release/stock_predictor .

# Set the entrypoint to run the server binary.
ENTRYPOINT ["./stock_predictor"]

# Expose the port.
EXPOSE 8080
