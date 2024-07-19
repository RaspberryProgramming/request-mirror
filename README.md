# Request Mirror
This application provides a web ui for sending get/post requests and provides a visual ui for looking at what the application received. This is written in rust.

![Build and Publish Docker Image](https://github.com/RaspberryProgramming/request-mirror/actions/workflows/docker-image.yml/badge.svg
)
![Build and Test Rust](https://github.com/RaspberryProgramming/request-mirror/actions/workflows/rust.yml/badge.svg)

## TODO:

N/a

## Docker

Please read through the documentation on setting up and installing docker on your machine.
We'll use the CLI commands to deploy the application to docker.

See [Get Docker](https://docs.docker.com/get-docker/)

First you'll want to ensure you have build the container. Do that by running

```bash
docker build . -t raspberrypi99/request-mirror
```

Next you can start up the application using docker compose

```bash
docker compose up -d
```

This will deploy the application to docker. It will setup the postgres server, deploy the database using diesel and start request-mirror.

The following command will stop the deployed containers

```bash
docker compose down
```

### Pushing to dockerhub

The image can then be pushed to docker using the following command. You may also want to modify it a little to your need

First login
```bash
docker login
```

Then push the image
```bash
docker push raspberrypi99/request-mirror:latest
```

## Development Environment

During development, you'll want to use a few tools to help work on this project.

First, you'll want to install docker.

For environments with a GUI: https://docs.docker.com/desktop/
For environments without a GUI: https://docs.docker.com/engine/install/

You can use docker for many things. This project has a set of files for creating and deploying as a docker container. In order to start development, you'll need a postgres container. By default in the .env file, the rust code will attempt to connect a localhost instance of postgres.

Use the following command to start a postgres container.

```bash
docker run --name postgres -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD=password -e POSTGRES_USER=postgres -e POSTGRES_DB=request_mirror_db -d postgres
```

This will start up an instance of postgres to only localhost. Remote computers can't connect to the database. This setup does not use any volumes, meaning that when the container is removed, the data will be gone. If you'd like to add volumes, you can run the following command instead which maps a new postgres-data volume to /var/lib/postgresql/data.

```bash
docker run --name postgres -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD=password -e POSTGRES_USER=postgres -e POSTGRES_DB=request_mirror_db -v postgres-data:/var/lib/postgresql/data  -d postgres:latest
```

Now that postgres is running, you can now install rust if you haven't already. Follow the instructions on the following site [rustup.rs](https://rustup.rs/)

Follow the instructions for installing the stable toolchain for rust. You may need to log in and log out after installation.

**Notice**: Older versions of request mirror used rocket v0.4.x which required the nightly toolchain, please update your repo/fork.

Next, we can install [diesel](https://diesel.rs/), an ORM and query builder for rust. This is how we deploy tables to our database. Install diesel cli:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

You can deploy the database with the following command:

**Notice**: If you are not adding a volume to your postgres database, you may need to re-run this step each time you create the postgres docker container.

```bash
diesel migration run
```

Now that the database is ready and rust is installed, we can move onto running the project.

```bash
cargo run
```

You can also run the following to run a release binary

```bash
cargo run --release
```

### Build Docker Image and Run Locally

You can build the docker image by running the following comand

```bash
docker build . -t raspberrypi99/request-mirror
```

Next, you can run the project using the following command. This can be run even with the development postgres container running. This will open port 80 for the user to connect to.

```bash
docker compose up -d
```