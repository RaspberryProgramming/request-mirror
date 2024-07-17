# Request Mirror
This application provides a web ui for sending get/post requests and provides a visual ui for looking at what the application received. This is written in rust.

![Docker Image CI](https://github.com/RaspberryProgramming/request-mirror/actions/workflows/docker-image.yml/badge.svg
)
![Build and Test](https://github.com/RaspberryProgramming/request-mirror/actions/workflows/rust.yml/badge.svg)

## TODO:
 - Update Readme
 - Document
 - Develop Pipelines

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

## Deploying to azure

TODO
