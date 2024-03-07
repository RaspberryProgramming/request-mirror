
# Set Rust to nightly
rustup default nightly

# Install Diesel
cargo install diesel_cli --no-default-features --features postgres

# New Network
docker network create postgres-net

# Setup postgres container
docker pull postgres
docker run -d --rm -p 5432:5432 --name postgres --net postgres-net -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password postgres

# Setup db with diesel
diesel database run
diesel migration run

# Build and start request-mirror
docker build . -t raspberrypi99/request-mirror
docker run -d --rm -p 8000:8000 --name request-mirror --net postgres-net raspberrypi99/request-mirror