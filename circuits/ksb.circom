pragma circom 2.2.1;

include "../node_modules/circomlib/circuits/poseidon.circom";

template KasbahCommitmentCircuit(DEPTH) {
    // Public Inputs
    signal input root;
    signal input nullifier_hash;

    // Private Inputs
    signal input nullifier;
    signal input secret;
    signal input merkle_path[DEPTH];
    signal input siblings[DEPTH];

    // 1. Compute the Commitment = Poseidon(nullifier, secret)
    component poseidonCommitment = Poseidon(2);
    poseidonCommitment.inputs[0] <== nullifier;
    poseidonCommitment.inputs[1] <== secret;

    signal output commitment;
    commitment <== poseidonCommitment.out;

    // 2. Verify the Merkle Path
    signal intermediateHashes[DEPTH + 1];
    signal left[DEPTH];
    signal right[DEPTH];
    signal not_merkle_path[DEPTH];
    signal prod_left1[DEPTH];
    signal prod_left2[DEPTH];
    signal prod_right1[DEPTH];
    signal prod_right2[DEPTH];

    // Initialize the first intermediate hash
    intermediateHashes[0] <== commitment;

    // Instantiate Poseidon hashers and compute the Merkle path
    component poseidonHashers[DEPTH];
    for (var i = 0; i < DEPTH; i++) {
        poseidonHashers[i] = Poseidon(2);
        
        // Since merkle_path and siblings are given top-to-bottom,
        // we need to access them in reverse order
        var pathIdx = DEPTH - 1 - i;
        
        // Ensure merkle_path[pathIdx] is binary (0 or 1)
        merkle_path[pathIdx] * (1 - merkle_path[pathIdx]) === 0;
        
        // Compute not_merkle_path using the reversed index
        not_merkle_path[i] <== 1 - merkle_path[pathIdx];
        
        // Compute left[i]
        prod_left1[i] <== intermediateHashes[i] * not_merkle_path[i];
        prod_left2[i] <== siblings[pathIdx] * merkle_path[pathIdx];
        left[i] <== prod_left1[i] + prod_left2[i];
        
        // Compute right[i]
        prod_right1[i] <== siblings[pathIdx] * not_merkle_path[i];
        prod_right2[i] <== intermediateHashes[i] * merkle_path[pathIdx];
        right[i] <== prod_right1[i] + prod_right2[i];
        
        // Compute the next hash
        poseidonHashers[i].inputs[0] <== left[i];
        poseidonHashers[i].inputs[1] <== right[i];
        intermediateHashes[i + 1] <== poseidonHashers[i].out;
    }

    // Output the root
    signal computedRoot;
    computedRoot <== intermediateHashes[DEPTH];

    // Constrain the computed root to be equal to the public input root
    computedRoot === root;

    // Compute and constrain Nullifier Hash
    component poseidonNullifierHash = Poseidon(1);
    poseidonNullifierHash.inputs[0] <== nullifier;
    poseidonNullifierHash.out === nullifier_hash;
}

component main {public [root, nullifier_hash]} = KasbahCommitmentCircuit(2);
