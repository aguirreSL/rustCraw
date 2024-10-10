# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/my_web_scraper

# Copy the entire project directory into the container
COPY . .

# Set the default command to run the Rust program
CMD ["cargo", "run", "--release"]
