# Reddit Community Points Demo Application

This project contains the demo application for Reddit.

Main features of it is to provide community tokens for users, register new communities
and manage subscriptions to these communities.

## Solution overflow

An architectural overview of the solution and all the involved parties can be found at the [workflow spec] docs section.

## Components

This repository contains two related but yet independent projects:

- [Community Oracle](community-oracle) - a bridge between zkSync network and the Reddit platform.
- [Service Provider](service-provider) - an application capable of initiating and storing the users' subscriptions.
- [`service-provider.js`](js) - a basic client TypeScript library for the application.

## Bootstrapping

To know how to bootstrap and run the application, see the [bootstrapping](./docs/bootstrapping.md) docs section.

## Service Provider API

To see the application API description, see the [Service Provider API](./docs/api_description.md) docs section.

# License

Reddit-demo project is distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.
