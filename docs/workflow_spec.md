# Reddit subscriptions workflow spec

This document provides an overview of the subscriptions mechanism implemented atop of the zkSync network.

## Involved parties

### User client

A client-side (e.g. front-end) code which converts desired user actions into requests for Service Provider and zkSync network.

### Community minting oracle

A web-server which provides information about the token distribution, in particular: amount of tokens to be minted by user.

### Service provider

An application which manages subscriptions. Service provider contains many "communities" (say, subreddits), each of them has its own token (ERC-20), name, subscription price, items price-list and list of subscribers.

### zkSync

Platform for performing tokens transfers with a speed higher than on L1 (Ethereum).

## General workflow overview

To retrieve community tokens, user client may request amount of tokens granted by the community. Then it must create a minting transaction (with user's signature) and request Service Provider to provide a second signature for this transaction. With two signatures acquired, user client may send the transaction to the zkSync network, and, once executed, community tokens will be minted to the user's account.

To initiate a subscription, user client creates a "subscription wallet": a zkSync account which will be only used to monitor the community subscription payments. Algorithm for creating a subscription wallet is deterministic and described below.

Once subscription wallet is created, user client creates a desired amount of pre-signed "subscription payment" transactions, which consist of the transfer to the subscription account, and the burn transaction (which will burn the payment for subscription).

Then, user client initiates a subscription by providing the community name, addresses of the user account and the subscription account, and pre-signed payment transactions.

Once the information is provided, user may check the subscription status. Service provider will execute an initial subscription transaction (for current month) in the zkSync network and, if the execution was successful, user will be considered subscribed to desired community.

## User client

### Deriving the private key for the subscription account

User must not store any keys other than the private key of their Ethereum account. In order to maintain this rule and make every subscription account per user/community pair unique, the keys for subscription accounts is derived as follows:

1. User encodes the hexadecimal value of their private key into a UTF-8 string.
2. User concatenates the obtained string value of their private key and the community name, prefixed by the domain (e.g. `reddit.com/r/rust`) (assuming that community names are unique).
3. User calculates the SHA256 hash of the obtained string.
4. If the calculated hash cannot be used as a Ethereum private key, user calculates the SHA256 hash of the hash value obtained previously. This step is repeated until the obtained value can be used as a private key.
5. Value, obtained on the step 4 is considered a private key for the community subscription account.

In a pseudocode the algorithm can be written as follows:

```python
def derive_private_key(private_key, prefixed_community_name):
    hex_private_key = encode_hex(private_key)
    concatenated = hex_private_key + prefixed_community_name
    private_key = sha256_hash(concatenated)
    while not can_be_used_as_private_key(private_key):
        private_key = h256_hash(private_key)

    return private_key
```

### Minting tokens

- User client can query the service provider for the details of token grant.
- Once the information about grant is received, user client can prepare a mint transaction and query service provider for a minting signature.
- User client now can send the transaction to the zkSync network.

### Creating a subscription

- User client must create a subscription wallet, using the algorithm described above.
- User client must initialize created account by sending a `ChangePubKey` transaction for this wallet.
- User client now can create subscription transactions and initiate subscription.

### Canceling a subscription

In order to cancel subscription, user may simply change the private key of the subscription account in order to invalidate remaining renewal transactions.

## Community minting oracle

The oracle is an application that governs token distribution. It provides minting signatures to the user, which will be used to mint tokens at the user's will.

### Token distribution

- The oracle calculates token distribution and prepares the list of users that should receive tokens.
- The oracle provides details of each token grant upon a user request and generates minting signatures upon request.

## Service provider

Service provider manages community subscriptions and provides an interface to users to create subscriptions and to use community tokens for various services.

### Creating a community

To create a community, Reddit must deploy the new ERC-20 token and provide the following information to the service provider:

- ERC20 token address
- ERC20 token name
- Community name
- Subscription cost(s) (assuming that community may have more than one subscription plan)

### API description

#### Public API

User can:

- Create subscription
- Check whether user is currently subscribed for a community service.
- Check the subscription payment status.

#### Private API

- Register a new community

### Monthly payments

To make the user's life simpler, Service Provider can take the responsibility to pay for the user's subscription on itself.

In order to do so, user must choose the period of the automatic prolongation (e.g. 12 months), and Service Provider will request user to sign a payment transaction for each month beforehand. These transactions will not be usual transfers, but rather a "delayed" transfers that can only be executed after reaching the certain time, and only valid for a certain period of time. For example, if user wants automatic subscription prolongation for 3 months from the Jan 1st to the Feb 28th, they'll be suggested to sign two transactions: "Transfer X community tokens Y to the subscription address Z at Jan 1st (tx is valid for 1 day)" and "Transfer X community tokens Y to the subscription address Z at Feb 1st (tx is valid for 1 day)".

These transactions will be stored in the database of the Service Provider, and will be sent to the zkSync network at the execution date.

Timestamp correctness is enforced by the cryptographic backed of the zkSync protocol, thus execution out of the agreed time is not possible.

If user cancels the subscription midway of the automatic prolongation period, remaining transactions become incorrect and will not affect the user's balance.

### Implementation details

Service Provider is a standalone application, which has its own database and provide several APIs (for public and private use).

Data stored in the Service Provider (off-chain):

- Communities info:
  - Community name.
  - Associated ERC-20 token.
  - Subscriptions plans.
  - Purchasable items.
- Users info:
  - Reddit username.
  - Address of the Ethereum account.
  - Address on the subscription account
  - List of active subscriptions along with pre-signed transactions.

Subscription is represented by the zkSync account owned by user solely.

Subscription is paid by performing a transfer to the subscription account & burning these funds right after.
Subscription is canceled is done via closing the subscription account. 

Items can be bought by burning the required amount of tokens and providing the corresponding tx hash to the Service Provider.

## zkSync

### New functionality

In order to suit the Reddit application needs, zkSync network introduces the following changes:

- A new operation is added: `TransferFrom`. This is an alternative form of transfer, which has the logic reversed from the `Transfer` operation: the funds are transfer **to** the sender's account. This kind of transaction affects the sender's nonce, and not the nonce of the account from which funds are transferred.
  *Reasoning:* This functionality is required to implement delayed transactions (for the automatic subscriptions prolongation).
  *Approximate form:* This structure should contain the fields `from` (account from which funds should be transferred), `token` (type of token for transfer), `amount` (amount of token to transfer), `execute_at` (timestamp for the moment when transaction can be executed), `valid_until` (the time range within which transaction can be executed), `nonce` (sender's account nonce), `fee` (zkSync fee to execute the transfer), `from_signature` (signature of the `from` account owner denoting the consent with funds being transferred from their account) and `sender_signature` (usual transaction signature proving the sender's account ownership).
- zkSync cCryptographic backend -- `circuit` module -- has the following new capabilities:
  - zkSync server is be able to provide an actual time to the circuit. This timestamp is stored in the circuit state and used as a reference for delayed transactions.
  - Constraints can verify the timestamp correctness. This includes the following checks: `tx_timestamp >= current_known_timestamp` and `current_timestamp <= (tx_timestamp + max_execution_time)`.
  - It is enforced that timestamps are monotonically growth.
- zkSync contract is now able verify the timestamps in the same way as in circuit (based on the `now` variable value in Solidity).
- zkSync network is now able to mint & burn funds.

### Workflow

Assuming that the service provider works as an independent application, the following workflow is assumed:

- zkSync network receives bundled transactions: `TransferFrom` to the subscription account and burn transaction for transferred funds.
- zkSync verifies correctness of these transactions, including the timestamps.
- Once transaction successfully executed, prover creates a proof for the block containing the transaction.
- Block data along with the proof is provided to the Ethereum smart contract, which verifies the generated proof.

Additionally, user can withdraw funds from their account to the Ethereum wallet using the same workflow as for any other withdrawal in zkSync network.

### A note on decentralization

The application itself stores a certain amount of information off-chain. This includes the following information:

- Community names and metadata.
- Usernames and addresses.

All the other data is stored in the zkSync blockchain, which is, on the other hand, is synchronized with the Ethereum chain.

Assuming that the community information and usernames are owned by Reddit, and without the Reddit the only available operation is to withdraw funds, the following problems and solutions are possible:

1. Service provider is down.
  **Solution:**
    - A new service provider is created.
    - Reddit provides all the information about its communities.
    - Users register themselves again and provide the subscriptions addresses they've 
    - For every registered user, zkSync blockchain is scanned and all the transactions are found.
    - Existing subscriptions are restored.
2. Service provider is down and zkSync network is down.
  **Solution:**
    - A new instance of zkSync server is created.
    - zkSync network state is restored using the Ethereum network (from the zkSync contract)
    - The remaining part of solution is the same as in the scenario 1.
3. Reddit, Service provider and zkSync network are down.
  **Solution:**
    - After contracts discovers that zkSync is unresponsive for a long time, it enters the exodus mode.
    - All the users can generate exit proofs and withdraw their tokens back to the Ethereum account.
