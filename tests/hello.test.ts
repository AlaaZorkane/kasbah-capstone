import { LAMPORTS_PER_SOL } from "./utils/constants.ts";
import {
  airdropFactory,
  appendTransactionMessageInstruction,
  createSignerFromKeyPair,
  createTransactionMessage,
  getSignatureFromTransaction,
  isSolanaError,
  lamports,
  pipe,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Address,
  type Blockhash,
  type KeyPairSigner,
} from "@solana/web3.js";
import { createLogger, explorerLocalUrl } from "./utils/helpers.ts";
import { assert, beforeAll, beforeEach, describe, it } from "vitest";
import {
  createLocalhostSolanaRpc,
  createLocalhostSolanaRpcSubscriptions,
} from "./__setup__.ts";
import { generateKeyPair } from "./utils/keypair.ts";
import { getHelloInstruction } from "../clients/js/src/generated/index.ts";

const log = createLogger("hello");

describe("kasbah hello instruction", () => {
  let rpc: ReturnType<typeof createLocalhostSolanaRpc>;
  let rpcSubscriptions: ReturnType<
    typeof createLocalhostSolanaRpcSubscriptions
  >;
  let airdrop: ReturnType<typeof airdropFactory>;
  let sendAndConfirm: ReturnType<typeof sendAndConfirmTransactionFactory>;
  let keypair: CryptoKeyPair;
  let signer: KeyPairSigner<string>;
  let signerPk: Address;
  let latestBlockhash: Readonly<{
    blockhash: Blockhash;
    lastValidBlockHeight: bigint;
  }>;

  beforeAll(async () => {
    rpc = createLocalhostSolanaRpc();
    rpcSubscriptions = createLocalhostSolanaRpcSubscriptions();
    airdrop = airdropFactory({ rpc, rpcSubscriptions });
    keypair = await generateKeyPair(true);
    signer = await createSignerFromKeyPair(keypair);
    signerPk = signer.address;
    sendAndConfirm = sendAndConfirmTransactionFactory({
      rpc,
      rpcSubscriptions,
    });

    await airdrop({
      commitment: "confirmed",
      lamports: lamports(LAMPORTS_PER_SOL * 1n),
      recipientAddress: signerPk,
    });
  });

  beforeEach(async () => {
    const { value } = await rpc
      .getLatestBlockhash({
        commitment: "confirmed",
      })
      .send();

    latestBlockhash = value;
  });

  it("successfully calls the hello function", async () => {
    const instruction = getHelloInstruction({
      id: 1,
      signer,
    });

    const txMsg = pipe(
      createTransactionMessage({
        version: 0,
      }),
      (tx) => setTransactionMessageFeePayerSigner(signer, tx),
      (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
      (tx) => appendTransactionMessageInstruction(instruction, tx),
    );

    const signedTx = await signTransactionMessageWithSigners(txMsg);

    const tx = getSignatureFromTransaction(signedTx);

    try {
      await sendAndConfirm(signedTx, {
        commitment: "confirmed",
      });
      log.info("signature: %s", tx);
      log.info("explorer url: %s", explorerLocalUrl(tx));
    } catch (error: unknown) {
      if (isSolanaError(error)) {
        log.error(error.context);
        assert.fail(error.message);
      }
    }
  });
});
