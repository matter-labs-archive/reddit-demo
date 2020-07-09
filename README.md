# Reddit Community Points Demo Application

This project contains the demo application for Reddit.

Main features of it is to provide community tokens for users, register new communities
and manage subscriptions to these communities.

## Bootstrapping

```sh
git clone git@github.com:matter-labs/reddit-demo.git 
cd reddit-demo
git submodule init --recursive
```

Note that since zkSync does not yet have a Rust client library published to the `crates.io`,
its main repository is added as a submodule to have access to its recent sources.

# License

Reddit-demo project is distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.
