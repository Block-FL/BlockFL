pragma circom 2.0.0;

template Rewards() {
    signal input X;   
    signal output Y[10]; 

    signal X_div_10;
    
    X_div_10 <== X / 10;
    
    for (var i = 0; i < 10; i++) {
        Y[i] <== X_div_10;
    }
}

component main = Rewards();
