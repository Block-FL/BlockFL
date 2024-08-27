#!/bin/bash

# File names
CIRCUIT_FILE="Rewards.circom"
WASM_DIR="Rewards_js"
INPUT_FILE="input.json"
WITNESS_FILE="witness.wtns"

# Check if circom is installed
if ! [ -x "$(command -v circom)" ]; then
  echo "Error: circom is not installed. Please install circom first." >&2
  exit 1
fi

# Compile the circuit
echo "Compiling the circuit..."
circom $CIRCUIT_FILE --r1cs --wasm --sym
if [ $? -ne 0 ]; then
  echo "Error: Compilation failed." >&2
  exit 1
fi

# Check if the input file exists
if [ ! -f $INPUT_FILE ]; then
  echo "Error: Input file $INPUT_FILE not found. Please create the input file." >&2
  exit 1
fi

# Generate the witness
echo "Generating the witness..."
node $WASM_DIR/generate_witness.js $WASM_DIR/Rewards.wasm $INPUT_FILE $WITNESS_FILE
if [ $? -ne 0 ]; then
  echo "Error: Witness generation failed." >&2
  exit 1
fi

echo "Witness generated successfully: $WITNESS_FILE"
