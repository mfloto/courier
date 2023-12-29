# The messenger

This project is meant to redirect emails together with their attachments into a discord channel via a webhook.

![screenshot of a sample email in discord](./images/testmail_discord.png)

## Getting started

1. Clone the repository (or at least the docker-compose.yml and config.toml)
2. Adjust the config.toml
3. Run `docker-compose up -d`

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/)
- [Docker-compose](https://docs.docker.com/compose/install/)

## TODO

- [x] Add DOCKERFILE and docker-compose
- [ ] Adjust docker-compose to point to image on dockerhub
- [ ] Configure CI
- [ ] Clean up code
- [ ] Add logging
- [ ] Add tests
- [ ] Add optional notification about errors on different discord channel