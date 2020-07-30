import {
    Provider as ZksyncProver, Wallet,
} from "zksync";
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

async function request(url: string): Promise<any> {
}

async function depositFunds(syncProvider: ZksyncProver): Promise<Wallet> {
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

async function initUserWallet(syncProvider: ZksyncProver, depositWallet: Wallet, seed: string): Promise<Wallet> {
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

(async () => {
    try {
        console.log("Starting the test");
        syncProvider = await ZksyncProver.newWebsocketProvider(process.env.WS_API_ADDR);

        console.log("Depositing funds");
        const depositWallet = await depositFunds(syncProvider);
        console.log("Creating a new user account");
        const userWallet = await initUserWallet(syncProvider, depositWallet, "01");

        // Check that user is not subscribed by default.
        console.log("Checking subscription status (not subscribed)");
        await checkSubscribed(userWallet, false);

        // Get the amount of granted tokens.
        console.log("Retrieving the granted tokens amount");
        const grantedTokens = await getGrantedTokensAmount(userWallet);



        console.log("Test completed");
        process.exit(0);
    } catch (e) {
        console.error("Error: ", e);
        process.exit(1);
    }
})();
