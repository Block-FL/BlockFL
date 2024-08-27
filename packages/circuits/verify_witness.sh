#!/bin/bash


# Step 1: Generate verification key
echo "Generating verification key..."
snarkjs powersoftau new bn128 14 pot14_final.ptau
snarkjs r1cs info circuits/Rewards.r1cs
snarkjs r1cs export json circuits/Rewards.r1cs circuits/Rewards.r1cs.json
snarkjs zkey new circuits/Rewards.r1cs pot14_final.ptau circuit_0000.zkey
snarkjs zkey contribute circuit_0000.zkey circuit_final.zkey
snarkjs zkey export verificationkey circuit_final.zkey verification_key.json

# Step 2: Verify witness
echo "Verifying witness..."
snarkjs wtns debug circuits/Rewards.wasm input.json witness.wtns public.json
snarkjs wtns calculate circuit_final.zkey witness.wtns proof.json public.json
snarkjs proof verify verification_key.json proof.json
