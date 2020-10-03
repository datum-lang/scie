const fromPairs = function(arr) {
    return arr.reduce(function(accumulator, value) {
        accumulator[value[0]] = value[1];
        return accumulator;
    }, {})
}
