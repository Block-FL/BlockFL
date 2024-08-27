pragma circom 2.1.8;

template Aggregator() {
    signal input X[10];
    signal sum;
    signal output mean;

    sum <-- X[0] + X[1] + X[2] + X[3] + X[4] + X[5] + X[6] + X[7] + X[8] + X[9];

    mean <== sum / 10;
}

component main = Aggregator();
