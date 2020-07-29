# Service Provider API

## Overview

Currently Service Provider has the following API structure:

- `/api/v0.1/related_communities` - get a list of communities related to the user.
- `/api/v0.1/is_user_subscribed` - check for users subscription status for a certain community.
- `/api/v0.1/subscribe` - initiate subscription by providing the address of the subscription wallet and pre-signed subscription transactions for several months.
- `/api/v0.1/granted_tokens` - check how many community tokens user can mint for a certain community.
- `/api/v0.1/get_minting_signature` - get a signature for the minting transaction.
- `/api/v0.1/genesis_wallet_address` - get a genesis wallet address.

Additional (logically private) API endpoints:

- `/api/v0.1/declare_commuity` - notify Service Provider about a new community created.

## Workflow

The expected client flow is the following:

1. User requests related communities.
2. For each returned community, user checks the amount of granted tokens.
3. User prepares the minting transactions for each community and provided amount of tokens.
4. User requests Service Provider to sign these transactions.
5. User executes signed minting transactions and get tokens.
6. User creates a subscription wallet and signs subscription transactions.
7. User initiates subscription by providing the subscription wallet address and pre-signed subscription txs to the Service Provider.

Endpoints involved in the process:
- Step 1: `related_communities` endpoint.
- Step 2: `granted_tokens` endpoint.
- Step 4: `get_minting_signature` endpoint.
- Step 7: `subscribe` endpoint.
- Remaining steps are done by user without Service Provider participation.

## Detailed description

This section provides a detailed description of inputs and outputs of the public API endpoints.

In case of any error on the Community Oracle side, the response will have a non-OK response HTTP code, and the response
body will match the following structure:

```typescript
{
    error: string // Occurred error description
}
```

### Important constants

For the demo purpose, the following values are hard-coded for interaction:

- Community name: "TestCommunity"
- Community token name: "MLTT"
- Amount of token granted to user (per request): 10_000 MLTT
- Cost of monthly subscription: 100 MLTT

### `genesis_wallet_address`

#### Description

Returns the address of the genesis wallet.

#### Input

```typescript
null
```

#### Output

```typescript
{
    address: string // Address of the genesis wallet
}
```

### `is_user_subscribed`

#### Description

Checks if user currently subscribed to the community (meaning that the subscription payment was done, and the subscription has not expired since).

#### Input

```typescript
{
    user: string, // Address of the user's main wallet.
    communityName: string, // Name of the community to be checked.
}
```

#### Output

```typescript
{
    subscribed: bool, // `true` if user is currently subscribed to the community, and `false` otherwise.
    startedAt?: string, // DateTime of the subscription period start.
    expiresAt?: string, // DateTime of the subscription period end.
}
```

### `subscribe`

Initiates a subscription by doing the following:

- Notifies the Community Oracle about the subscription wallet for community created by user.
- Adds the pre-signed transactions for subscription payment.

Sample usage code (assuming using the most recent `zksync.js` version):

```typescript
const subscriptionMonths = 12;
const subscriptionTransactions = await wallet.createSubscriptionTransactions(subscriptionWallet, subscriptionMonths);

const subscriptionRequest = {
    "user": wallet.address(),
    "communityName": "TestCommunity",
    "subscriptionWallet": subscriptionWallet.address(),
    txs: subscriptionTransactions
};
```

#### Input

```typescript
{
    user: string, // Address of the user's main wallet.
    communityName: string, // Name of the community to be checked.
    subscriptionWallet: string, // Address of the subscription wallet.
    txs: SubscriptionTx[], // List of the pre-signed txs to pay for subscription.
}
```

Subscription Tx is defined as follows: 

```typescript
{
    transferToSub: TransferFrom,
    burnTx: Transfer,
    burnTxEthSignature: string,
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
    communityName: string, // Name of the community to be checked.
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

To use this method, a minting transaction must be initially created. 

Sample usage code (assuming using the most recent `zksync.js` version):

```typescript
// As the "from" signature is currently unknown, initialize it as an empty signature.
let fromSignature = {
    "pubKey":"0000000000000000000000000000000000000000000000000000000000000000",
    "signature":"00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
}; 
let mintingTx = await wallet.createTransferFromNoSend({
    from: genesisWalletAddress,
    token: "MLTT",
    amount: 10000,
    fee: transferFromFee,
    nonce: walletNonce,
    validFrom, // Should be equal to the current time, so it may be executed right after signing.
    validUntil, // Better to set as "validFrom + 1 day"
    fromSignature,
});

mintingTx.fromSignature = ...; // Now that we have a `TransferFrom` object, we can request minting signature from the Service Provider.

// Submit signed minting tx.
const submitResponse = await wallet.provider.submitTx(signedWithdrawTransaction);

// If required, we can create a "Transaction" object.
const transaction = new Transaction(
    mintingTx,
    submitResponse,
    wallet.provider
);
```

#### Input

```typescript
{
    user: string, // Address of the user's main wallet.
    communityName: string, // Name of the community to be checked.
    mintingTx: TransferFrom, // Created, but not signed minting transaction.
}
```

#### Output

```typescript
{
    signature: { zksyncSignature: Signature } // Signature for a minting transaction in a hexadecimal form.
}
```

