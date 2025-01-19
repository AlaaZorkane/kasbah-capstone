import { assertKeyGenerationIsAvailable } from "@solana/assertions";

// @see https://github.com/anza-xyz/solana-web3.js/issues/47#issuecomment-2576890160
export async function generateKeyPair(
  extractable = false,
): Promise<CryptoKeyPair> {
  await assertKeyGenerationIsAvailable();
  const keyPair = await crypto.subtle.generateKey(
    /* algorithm */ "Ed25519", // Native implementation status: https://github.com/WICG/webcrypto-secure-curves/issues/20
    /* extractable */ extractable, // Prevents the bytes of the private key from being visible to JS.
    /* allowed uses */ ["sign", "verify"],
  );
  return keyPair;
}

export async function getPrivateKeyFromKeypair(
  keypair: CryptoKeyPair,
): Promise<Uint8Array> {
  const rawPrivateKey = await crypto.subtle.exportKey(
    "pkcs8",
    keypair.privateKey,
  );
  const publicKey = await crypto.subtle.exportKey("raw", keypair.publicKey);
  const keyBytes = new Uint8Array(rawPrivateKey);

  // Extract the 32-byte seed from PKCS#8 format
  const seed = keyBytes.slice(16, 48);

  // Create the 64-byte expanded private key
  const expandedPrivateKey = new Uint8Array(64);

  // Copy the original 32-byte seed
  expandedPrivateKey.set(seed, 0);

  // Set the public key (32 bytes)
  expandedPrivateKey.set(new Uint8Array(publicKey), 32);

  return expandedPrivateKey;
}
