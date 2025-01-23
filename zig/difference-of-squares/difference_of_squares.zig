const std = @import("std");

pub fn squareOfSum(number: usize) usize {
    var sum: usize = 0;

    for (0..number) |i| {
        sum += (i + 1);
    }

    return sum * sum;
}

pub fn sumOfSquares(number: usize) usize {
    var sum: usize = 0;

    for (0..number) |i| {
        const naturalNumber = i + 1;
        sum += (naturalNumber * naturalNumber);
    }

    return sum;
}

pub fn differenceOfSquares(number: usize) usize {
    const _squareOfSum = squareOfSum(number);
    const _sumOfSquares = sumOfSquares(number);

    return _squareOfSum - _sumOfSquares;
}
