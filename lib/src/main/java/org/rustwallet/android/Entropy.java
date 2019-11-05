package org.rustwallet.android;

public enum Entropy {

    Low(16),
    Recommended(32),
    Paranoid(64);

    int n;

    Entropy(int n) {
        this.n = n;
    }
}
