import {
    Provider, Wallet,
} from "zksync";
import {ethers} from "ethers";
import {parseEther} from "ethers/utils";
import * as types from "../src/types";

const SERVICE_PROVIDER_URL = "http://127.0.0.1:8080/";

const WEB3_URL = process.env.WEB3_URL;

const network = process.env.ETH_NETWORK == "localhost" ? "localhost" : "stage";
const ethersProvider = new ethers.providers.JsonRpcProvider(WEB3_URL);
if (network == "localhost") {
    ethersProvider.pollingInterval = 100;
}

let syncProvider: Provider;

async function request(url: string): Promise<any> {
}

async function depositFunds(syncProvider: Provider): Promise<Wallet> {
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

async function initUserWallet(syncProvider: Provider, depositWallet: Wallet, seed: string): Promise<Wallet> {
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

// async function checkSubscribed(provider: Provider, 

(async () => {
    try {
        syncProvider = await Provider.newWebsocketProvider(process.env.WS_API_ADDR);

        const depositWallet = await depositFunds(syncProvider);
        const userWallet = await initUserWallet(syncProvider, depositWallet, "01");
    } catch (e) {
        console.error("Error: ", e);
        process.exit(1);
    }
})();
