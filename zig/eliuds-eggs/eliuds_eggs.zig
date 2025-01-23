const std = @import("std");

pub fn eggCount(number: usize) usize {
    return @popCount(number);
}
