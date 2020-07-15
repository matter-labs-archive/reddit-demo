# Service Provider API

## Overview

Currently Service Provider has the following API structure:

- `/api/v0.1/related_communities` - get a list of communities related to the user.
- `/api/v0.1/is_user_subscribed` - check for users subscription status for a certain community.
- `/api/v0.1/set_subscription_info` - initiate the subscription process by notifying the Service Provider about created subscription wallet.
- `/api/v0.1/add_subscription_txs` - add the pre-signed subscription transactions for automatic subscription prolongation.
- `/api/v0.1/granted_tokens` - check how many community tokens user can mint for a certain community.
- `/api/v0.1/get_minting_signature` - get a signature for the minting transaction.

Additional (logically private) API endpoints:

- `/api/v0.1/declare_commuity` - notify Service Provider about a new community created.

## Workflow

The expected client flow is the following:

1. User requests related communities.
2. For each returned community, user checks the amount of granted tokens.
3. User prepares the minting transactions for each community and provided amount of tokens.
4. User requests Service Provider to sign these transactions.
5. User executes signed minting transactions and get tokens.
6. User initiates subscriptions by creating corresponding wallets and notifying the Service Provider about them.
7. For each subscription, user signs subscription transactions and provides them to the Service Provider.

Endpoints involved in the process:
- Step 1: `related_communities` endpoint.
- Step 2: `granted_tokens` endpoint.
- Step 4: `get_minting_signature` endpoint.
- Step 6: `set_subscription_info` endpoint.
- Step 7: `add_subscription_txs` endpoint.
- Remaining steps are done by user without Service Provider participation.

## Detailed description

This section provides a detailed description of inputs and outputs of the public API endpoints.

**Note:** Several endpoints contain field `auth` which has the `string` type. This field is to be replaced with the
actual authentication data, such as access token, but currently is not checked and can be empty (for the demo purposes).

In case of any error on the Community Oracle side, the response will have a non-OK response HTTP code, and the response
body will match the following structure:

```typescript
{
    error: string // Occurred error description
}
```

### `related_communities`

#### Description

Returns the list of communities relevant to the user (e.g. ones that can grant them tokens). User may be not subscribed to these
communities, for subscription check see the `is_user_subscribed` endpoint.

#### Input

```typescript
{
    user: string, // Address of the user's main wallet.
    auth: string // Authorization data. Currently can be empty string.
}
```

#### Output

```typescript
{
    [index: number]: string // List of related community names
}
```

### `is_user_subscribed`

#### Description

Checks if user currently subscribed to the community (meaning that the subscription payment was done, and the subscription has not expired since).

#### Input

```typescript
{
    user: string, // Address of the user's main wallet.
    community_name: string, // Name of the community to be checked.
    auth: string // Authorization data. Currently can be empty string.
}
```

#### Output

```typescript
{
    subscribed: bool // `true` if user is currently subscribed to the community, and `false` otherwise.
}
```

### `set_subscription_info`

Notifies the Community Oracle about the subscription wallet for community created by user.

#### Input

```typescript
{
    user: string, // Address of the user's main wallet.
    community_name: string, // Name of the community to be checked.
    subscription_wallet: string, // Address of the subscription wallet.
    auth: string // Authorization data. Currently can be empty string.
}
```

#### Output

```typescript
null
```

### `add_subscription_txs`

Adds the pre-signed transactions for subscription payment.

**Note:** As the API for subscribing is not yet implemented, the structure of the `SubscriptionTx` type is currently **unknown**.
This document will be updated with the required type definition once it's designed.

#### Input

```typescript
{
    user: string, // Address of the user's main wallet.
    community_name: string, // Name of the community to be checked.
    [index: number]: SubscriptionTx, // List of the pre-signed txs to pay for subscription.
    auth: string // Authorization data. Currently can be empty string.
}
```

#### Output

```typescript
null
```

### `granted_tokens`

#### Description

Returns the type and amount of community tokens that user can mint.

#### Input

```typescript
{
    user: string, // Address of the user's main wallet.
    community_name: string, // Name of the community to be checked.
    auth: string // Authorization data. Currently can be empty string.
}
```

#### Output

```typescript
{
    token: string, // Name of the community token
    amount: number // Amount of tokens user can mint
}
```


### `get_minting_signature`

#### Description

Checks that user provided a correct minting transaction, and provides a signature for it.

**Note:** As the API for subscribing is not yet implemented, the structure of the `MintingTransaction` type is currently **unknown**.
This document will be updated with the required type definition once it's designed.

#### Input

```typescript
{
    user: string, // Address of the user's main wallet.
    community_name: string, // Name of the community to be checked.
    minting_tx: MintingTransaction, // Created, but not signed minting transaction.
    auth: string // Authorization data. Currently can be empty string.
}
```

#### Output

```typescript
{
    signature: string // Signature for a minting transaction in a hexadecimal form.
}
```

