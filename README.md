# rust-web-services

This project implements the same API surface which was implemented in the Pluralsight course "Creating Web Services with Go" by Alex Schultz, with Rust using the Rocket web framework.

## The plan

- Cosmos DB
- CRUD API with Azure Functions
- API Management
- Frontend hosted... somehow

## How-To

Some things are automated with [`just`](https://github.com/casey/just#what-are-the-idiosyncrasies-of-make-that-just-avoids). Just run just for a menu.

```sh
# Initialize/Reset a local Database instance
just init-db

# Run cargo-watch against a compiled binary
just watch-api

# Run locally as an Azure Function
just start-function
```

Requests in `requests.http` are compatibile with the [VSCode REST Client extension](https://marketplace.visualstudio.com/items?itemName=humao.rest-client). You can use them against the API.
