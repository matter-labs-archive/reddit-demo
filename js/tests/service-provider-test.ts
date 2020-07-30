import {
    Provider as ZksyncProver, Wallet, types as zksyncTypes,
} from "zksync";
import * as zksync from "zksync";
import {ethers} from "ethers";
import {parseEther} from "ethers/utils";
import * as types from "../src/types";
import { Provider as ServiceProvider } from "../src/provider";

const SERVICE_PROVIDER_URL = "http://127.0.0.1:8080/";
const COMMUNITY_NAME = "TestCommunity";
const COMMUNITY_TOKEN = "MLTT";
const GRANTED_TOKENS_AMOUNT = 10_000;

const WEB3_URL = process.env.WEB3_URL;

const network = process.env.ETH_NETWORK == "localhost" ? "localhost" : "stage";
const ethersProvider = new ethers.providers.JsonRpcProvider(WEB3_URL);
if (network == "localhost") {
    ethersProvider.pollingInterval = 100;
}

let syncProvider: ZksyncProver;

async function depositFunds(): Promise<Wallet> {
    const depositEthWallet = ethers.Wallet.fromMnemonic(
        process.env.TEST_MNEMONIC, "m/44'/60'/0'/0/5"
    ).connect(ethersProvider);
    const depopsitSyncWallet = await Wallet.fromEthSigner(depositEthWallet, syncProvider);

    // PARAMS
    const depositAmount = parseEther("100000");
    const ERC20_SYMBOL = "MLTT";
    const ERC20_ID = syncProvider.tokenSet.resolveTokenId(ERC20_SYMBOL);

    // Deposit phase
    const deposit = await depopsitSyncWallet.depositToSyncFromEthereum({
        depositTo: depopsitSyncWallet.address(),
        token: ERC20_SYMBOL,
        amount: depositAmount,
        approveDepositAmountForERC20: true
    });
    await deposit.awaitReceipt();

    if (!await depopsitSyncWallet.isSigningKeySet()) {
        const changePubkey = await depopsitSyncWallet.setSigningKey();
        await changePubkey.awaitReceipt();
    }

    return depopsitSyncWallet;
}

async function initUserWallet(depositWallet: Wallet, seed: string): Promise<Wallet> {
    const userEthWallet = ethers.Wallet.fromMnemonic(
        process.env.TEST_MNEMONIC, "m/44'/60'/0'/0/5" + seed
    ).connect(ethersProvider);
    const userSyncWallet = await Wallet.fromEthSigner(userEthWallet, syncProvider);

    // PARAMS
    const transferAmount = parseEther("99");
    const ERC20_SYMBOL = "MLTT";
    const ERC20_ID = syncProvider.tokenSet.resolveTokenId(ERC20_SYMBOL);

    // Transfer phase
    const fullFee = await syncProvider.getTransactionFee("Transfer", userSyncWallet.address(), ERC20_SYMBOL);
    const fee = fullFee.totalFee;

    const transfer = await depositWallet.syncTransfer({
        to: userSyncWallet.address(),
        token: ERC20_SYMBOL,
        amount: transferAmount,
        fee
    });
    await transfer.awaitReceipt();

    if (!await userSyncWallet.isSigningKeySet()) {
        const changePubkey = await userSyncWallet.setSigningKey();
        await changePubkey.awaitReceipt();
    }

    return userSyncWallet;
}

async function checkSubscribed(userWallet: Wallet, expectedStatus: boolean) {
    const serviceProvider = new ServiceProvider(SERVICE_PROVIDER_URL);

    const response = await serviceProvider.isUserSubscribed(userWallet.address(), COMMUNITY_NAME);

    if (response.subscribed != expectedStatus) {
        throw Error(`User subscription status incorrect: expected ${expectedStatus}, actual: ${response.subscribed}`);
    }
}

async function getGrantedTokensAmount(userWallet: Wallet): Promise<types.GrantedTokensResponse> {
    const serviceProvider = new ServiceProvider(SERVICE_PROVIDER_URL);

    const response = await serviceProvider.grantedTokens(userWallet.address(), COMMUNITY_NAME);

    if (response.token != COMMUNITY_TOKEN) {
        throw Error(`Granted token type incorrect: expected ${COMMUNITY_TOKEN}, actual: ${response.token}`);
    }

    if (response.amount != GRANTED_TOKENS_AMOUNT) {
        throw Error(`Granted token amount incorrect: expected ${GRANTED_TOKENS_AMOUNT}, actual: ${response.amount}`);
    }

    return response;
}

async function mintTokens(userWallet: Wallet, communityName: string, token: string, amount: number) {
    const serviceProvider = new ServiceProvider(SERVICE_PROVIDER_URL);

    const fullFee = await syncProvider.getTransactionFee("TransferFrom", userWallet.address(), token);
    const transferFromFee = fullFee.totalFee;

    const genesisAddress = await serviceProvider.genesisWalletAddress();
    console.log(`Genesis address is ${genesisAddress}`)
    const walletNonce = await userWallet.getNonce();

    const subPeriodSeconds = 31 * 24 * 3600; // 31 days * 24 hours * 3600 seconds.
    const oneDay = 24 * 3600; // 24 hours * 3600 seconds.
    const currentUnixTime = Math.floor(Date.now() / 1000) - 60; // Subtract one minute to cover the difference between server/client.

    const validFrom = currentUnixTime;
    const validUntil = currentUnixTime + oneDay;

    // As the "from" signature is currently unknown, initialize it as an empty signature.
    let fromSignature = {
        "pubKey":"0000000000000000000000000000000000000000000000000000000000000000",
        "signature":"00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
    }; 
    let mintingTx = await userWallet.createTransferFromNoSend({
        from: genesisAddress,
        token,
        amount,
        fee: transferFromFee,
        nonce: walletNonce,
        validFrom,
        validUntil,
        fromSignature,
    });

    // Now that we have a `TransferFrom` object, we can request minting signature from the Service Provider.
    mintingTx.fromSignature = await serviceProvider.getMintingSignature(userWallet.address(), communityName, mintingTx);

    // Submit signed minting tx.
    const submitResponse = await userWallet.provider.submitTx(mintingTx);

    // If required, we can create a "Transaction" object.
    const transaction = new zksync.Transaction(
        mintingTx,
        submitResponse,
        userWallet.provider
    );

    await transaction.awaitReceipt();
}

async function createSubscriptionWallet(userWallet: Wallet, communityName: string): Promise<Wallet> {
    const postfix = "reddit.com/r/" + communityName;
    const subscriptionWallet = await userWallet.createDerivedWallet(postfix);

    // Initialize the wallet via `ChangePubKey` operation.
    let changePubkeyTx = await subscriptionWallet.setSigningKey();
    await changePubkeyTx.awaitReceipt();

    let accountId = await subscriptionWallet.getAccountId();
    console.log(`Subscription accound ID is ${accountId}`);

    return subscriptionWallet;
}

async function subscribe(userWallet: Wallet, subscriptionWallet: Wallet, communityName: string) {
    const serviceProvider = new ServiceProvider(SERVICE_PROVIDER_URL);

    const months = 12;
    const subscriptionTxs = await userWallet.createSubscriptionTransactions(subscriptionWallet, months);

    await serviceProvider.subscribe(userWallet.address(), communityName, subscriptionWallet.address(), subscriptionTxs);
}

(async () => {
    try {
        console.log("Starting the test");
        syncProvider = await ZksyncProver.newWebsocketProvider(process.env.WS_API_ADDR);

        console.log("Depositing funds");
        const depositWallet = await depositFunds();
        console.log("Creating a new user account");
        const userWallet = await initUserWallet(depositWallet, "01");

        // Check that user is not subscribed by default.
        console.log("Checking subscription status (not subscribed)");
        await checkSubscribed(userWallet, false);

        // Get the amount of granted tokens.
        console.log("Retrieving the granted tokens amount");
        const grantedTokens = await getGrantedTokensAmount(userWallet);

        // Mint granted tokens.
        console.log("Minting community tokens");
        await mintTokens(userWallet, COMMUNITY_NAME, grantedTokens.token, grantedTokens.amount);

        // Create a subscription wallet.
        console.log("Creating a subscription wallet");
        const subscriptionWallet = await createSubscriptionWallet(userWallet, COMMUNITY_NAME);

        // Subscribe to the community.
        console.log("Subscribing user");
        await subscribe(userWallet, subscriptionWallet, COMMUNITY_NAME);

        // User now should be subscribed.
        console.log("Checking subscription status (subscribed)");
        await checkSubscribed(userWallet, true);

        console.log("Test completed");
        process.exit(0);
    } catch (e) {
        console.error("Error: ", e);
        process.exit(1);
    }
})();
