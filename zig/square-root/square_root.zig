pub fn squareRoot(radicand: usize) usize {
    for (1..radicand + 1) |x| {
        if (x * x == radicand) {
            return x;
        }
    }

    return 0;
}
