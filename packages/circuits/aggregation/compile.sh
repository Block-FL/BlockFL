#!/bin/bash

# Step 1: Circuit Compilation
echo "Compiling circuit..."
circom Aggregator.circom --r1cs --wasm --sym

# Checking if compilation was successful
if [ $? -ne 0 ]; then
    echo "Compilation failed. Please check the Circom circuit."
    exit 1
fi

# Step 2: Generating witness
echo "Generating witness..."
echo "{\"X\": [$1, $2, $3, $4, $5, $6, $7, $8, $9, ${10}]}" > input.json
snarkjs calculatewitness --wasm ./Aggregator_js/Aggregator.wasm input.json witness.wtns

# Checking if witness generation was successful
if [ $? -ne 0 ]; then
    echo "Witness generation failed."
    exit 1
fi

# Step 3: Serving witness file for browser display
mkdir -p public
cp witness.wtns public/witness.wtns

echo "Witness generated successfully!"
