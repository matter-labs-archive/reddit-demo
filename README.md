# Reddit Community Points Demo Application

This project contains the demo application for Reddit.

Main features of it is to provide community tokens for users, register new communities
and manage subscriptions to these communities.

## Components

This repository contains two related but yet independent projects:

- [Community Oracle](community-oracle) - a bridge between zkSync network and the Reddit platform.
- [Service Provider](service-provider) - an application capable of initiating and storing the users' subscriptions.

## Bootstrapping

### Initializing the repo

```sh
git clone git@github.com:matter-labs/reddit-demo.git 
cd reddit-demo
git submodule init --recursive
```

Note that since zkSync does not yet have a Rust client library published to the `crates.io`,
its main repository is added as a submodule to have access to its recent sources.

### Running the Community Oracle

```sh
cd community-oracle
cargo run
```

The commands above will start a new instance of the Community Oracle with a server running on the `127.0.0.1:4040`.

The API of Community Oracle is not public and doesn't need to be exposed to the web.

### Running the Community Oracle

Before running the Service Provider, you will have to edit `service-provider/config.json` file.

It has the following variables:

- `genesis_account_id`: zkSync ID of the genesis account,
- `genesis_account_address`: zkSync address of the genesis account,
- `genesis_account_private_key`: zkSync private key of the genesis account,
- `genesis_account_eth_private_key`: ethereum private key of the genesis account.

After editing, you can run the application as follows:

```sh
cd community-oracle
cargo run
```

The commands above will start a new instance of the Community Oracle with a server running on the `127.0.0.1:4040`.

The API of Community Oracle is not public and doesn't need to be exposed to the web.

Alternatively, config for the application can be loaded from the environment variables.
To do so, simply add the `--env_config` flag when running the application.

### Running the Service Provider

Before running the Service Provider, you will have to edit `service-provider/config.json` file.

It has the following variables:

- `app_bind_address`: address on which Service Provider server will be listening. Default is "127.0.0.1:8080",
- `zksync_rest_api_address`: address of the zkSync server REST API. Currently not used. Default is "http://127.0.0.1:3001",
- `zksync_json_rpc_address`: address of the zkSync server HTTP JSON RPC. Currently not used. Default is "http://127.0.0.1:3030",
- `community_oracle_address`: address of the Community Oracle API. Default is "http://127.0.0.1:4040",
- `burn_account_address`: address of the account to burn funds to.

After editing, you must run the `community-oracle` binary, and then you can run the application as follows:

```sh
cd service-provider
cargo run
```

To check whether application runs correctly, you may perform the following query (assuming that provider is running on `127.0.0.1:8080`):

```sh
curl --header "Content-Type: application/json" \
    --header "Accept: application/json" \
    --request POST \
    --data '{"user": "0x69b51c86056fbc4f4a733b25533072b6cbbe3a21" } ' \
    http://127.0.0.1:8080/api/v0.1/related_communities
```

If everything is OK, the following response will be returned:

```js
{"communities":["TestCommunity"]}
```

Alternatively, config for the application can be loaded from the environment variables.
To do so, simply add the `--env_config` flag when running the application.

## TypeScript client library

This application has a very basic (currently) client library implemented in TypeScript.

You can find it description in the [js folder](./js) of this repository.

# License

Reddit-demo project is distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.
