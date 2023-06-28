FROM rust:1.67

# Build the server binary.
WORKDIR /usr/src/stock_predictor
COPY . .
RUN cargo build --release

# Set the entrypoint to run the server binary.
ENTRYPOINT ["./target/release/stock_predictor"]

# Expose the port.
EXPOSE 8080
