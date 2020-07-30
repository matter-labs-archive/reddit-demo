# `service-provider.js`

A work-in-progress typescript client library for the Reddit service provider.

Basic usage:

```typescript
const serviceProvider = new ServiceProvider(SERVICE_PROVIDER_URL);

const response = await serviceProvider.isUserSubscribed(userWallet.address(), COMMUNITY_NAME);

if (response.subscribed) {
    console.log(`User is subscribed to the community ${COMMUNITY_NAME}.`);
    console.log(`Subscription start: ${response.startedAt}. Subscription expires: ${response.expiresAt}`);
} else {
    console.log(`User is not subscribed to the community ${COMMUNITY_NAME}.`);
}
```

A more verbose real-life usage example can be found at the [`service-provider-test.ts`](./tests/service-provider-test.ts) script.
