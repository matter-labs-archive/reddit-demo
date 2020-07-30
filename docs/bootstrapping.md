### Initializing the repo

```sh
git clone git@github.com:matter-labs/reddit-demo.git 
cd reddit-demo
git submodule init --recursive
```

Note that since zkSync does not yet have a Rust client library published to the `crates.io`,
its main repository is added as a submodule to have access to its recent sources.

### Prerequisites

To get this application running, you will need the running zkSync server.

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
