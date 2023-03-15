# starter app for rust + Rocket + fly.io

A minimal [Rocket](https://rocket.rs/) application with the necessary config to deploy to [fly.io](https://fly.io).

The Dockerfile uses [cargo chef](https://crates.io/crates/cargo-chef) for layer caching.

It also includes github actions for automatic deploy on push to main branch.

## Setup

Create a new repo, copy the contents of this repo into the new repo.

Set up github actions by running the following command:

```
$ flyctl auth login
$ gh secret set --body $(flyctl auth token) --repo your-username/your-repo-name FLY_API_TOKEN
```

And then make change, commit and push, and watch the deployment action run.
