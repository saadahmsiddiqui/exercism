// Please implement the `ComputationError.IllegalArgument` error.
pub const ComputationError = error{IllegalArgument};

pub fn steps(number: usize) anyerror!usize {
    if (number == 0) return ComputationError.IllegalArgument;

    var _steps: usize = 0;
    var _mutable_number: usize = number;

    while (_mutable_number != 1) {
        if (_mutable_number % 2 == 0) {
            _mutable_number = _mutable_number / 2;
        } else {
            _mutable_number = _mutable_number * 3 + 1;
        }

        _steps += 1;
    }

    return _steps;
}
