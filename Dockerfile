FROM rust:latest

# Install the necessary dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Create a new directory for the application
RUN mkdir /app
WORKDIR /app

# Copy the application code into the container
COPY . .

# Build the application
RUN cargo build --release

# Set the application as the entrypoint
ENTRYPOINT ["/app/target/release/twitter-like-service"]