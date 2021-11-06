# Warden

Warden is a dynamic task runner service using webhook events. It serves as a continuous integration
and deployment utility service by helping to automate custom pipelines.

## Roadmap

### Version 0.1.0

The following list of features is mostly complete and does not contain the full list of features
that will be present in Warden 0.1.0.

- [X] Configuration
  - [X] Read YAML file at startup to populate config
  - [X] Environment-based configuration (i.e. development, staging, production)
  - [X] Override config values with environment variables
- [X] Docker support
  - [X] Dockerfile
  - [ ] Docker-Compose support for running in standalone mode
- [ ] Runtime
  - [X] Asynchronous IO
  - [ ] Multithreading for parallel processing
  - [ ] Load and execute local scripts
- [ ] Webhook-based task runner
  - [ ] VCS notifications via GitHub webhooks
  - [ ] Parse notification content and sender
  - [ ] Secure endpoints with secrets and API tokens
- [ ] CI/CD pipeline
  - [ ] Set up initial continuous integration with a TravisCI
  - [ ] Add workflows/jobs for every push to `main`
  - [ ] Set up CD for automated deployment

## Licensing

The project is licensed under the [MIT LICENSE](LICENSE)
