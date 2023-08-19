let fibonacci = fn(x) {
  let rec = fn(x, acc) {
    if (x == 0) {
      return acc;
    } else {
      rec(x - 1, push(acc, acc[-1] + acc[-2]));
    }
  };
  rec(x, [0, 1]);
};

puts(fibonacci(50));