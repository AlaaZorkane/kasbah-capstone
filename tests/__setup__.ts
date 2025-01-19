import {
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  devnet,
} from "@solana/web3.js";

export function createLocalhostSolanaRpc() {
  return createSolanaRpc(devnet("http://127.0.0.1:8899"));
}

export function createLocalhostSolanaRpcSubscriptions() {
  return createSolanaRpcSubscriptions(devnet("ws://127.0.0.1:8900"));
}
