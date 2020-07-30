import { utils } from "ethers";
import * as zksync from "zksync";

export interface SubscriptionCheckResponse {
    // `true` if user is currently subscribed to the community, and `false` otherwise.
    subscribed: boolean;
    // DateTime of the subscription period start.
    startedAt?: string;
    // DateTime of the subscription period end.
    expiresAt?: string;
}

export interface GenesisAddressResponse {
    // Address of the genesis wallet
    address: string;
}

export interface GrantedTokensResponse {
    // Name of the community token
    token: string;
    // Amount of tokens user can mint
    amount: number;
}

export interface MintedSignatureResponse {
    // Signature for a minting transaction.
    signature: { zksyncSignature: zksync.types.Signature }
}

export interface SubscriptionTx {
    transferToSub: zksync.types.TransferFrom,
    burnTx: zksync.types.Transfer,
    burnTxEthSignature: zksync.types.TxEthSignature,
}
