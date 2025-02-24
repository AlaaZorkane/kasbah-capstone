pragma circom 2.2.1;

include "../node_modules/circomlib/circuits/poseidon.circom";

template KasbahCommitmentCircuit(DEPTH) {
    // Public Inputs
    signal input root;
    signal input nullifier_hash;

    // Private Inputs
    signal input nullifier;
    signal input secret;
    signal input path_indices[DEPTH];
    signal input merkle_path[DEPTH];

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
    signal not_path_indices[DEPTH];
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

        // Ensure path_indices[i] is binary (0 or 1)
        path_indices[i] * (1 - path_indices[i]) === 0;

        // Compute not_path_indices[i]
        not_path_indices[i] <== 1 - path_indices[i];

        // Compute left[i]
        prod_left1[i] <== intermediateHashes[i] * not_path_indices[i];
        prod_left2[i] <== merkle_path[i] * path_indices[i];
        left[i] <== prod_left1[i] + prod_left2[i];

        // Compute right[i]
        prod_right1[i] <== merkle_path[i] * not_path_indices[i];
        prod_right2[i] <== intermediateHashes[i] * path_indices[i];
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

component main {public [root, nullifier_hash]} = KasbahCommitmentCircuit(20);
