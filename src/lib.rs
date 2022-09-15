pub fn reverse_string(string: &mut str) {
    // Safety: the slice will be valid UTF-8 by the time the borrow ends
    let slice = unsafe { string.as_bytes_mut() };

    let mut cur_index = 0;
    while cur_index < slice.len() {
        // Safety: anything in `slice` at `cur_index` or after has not been
        // touched, so it's valid UTF-8
        let str_representation = unsafe { std::str::from_utf8_unchecked(&slice[cur_index..]) };

        // Safety: `str_reprentation` has at least one character since it's
        // valid UTF-8 and is not empty (since `cur_index < slice.len()`), so we
        // can unwrap
        let cur_char = unsafe { str_representation.chars().next().unwrap_unchecked() };
        let len_cur_char = cur_char.len_utf8();

        slice[cur_index..(cur_index + len_cur_char)].reverse();
        cur_index += len_cur_char;
    }

    slice.reverse();
}

pub fn max(i1: i32, i2: i32, i3: i32) -> i32 {
    i1.max(i2).max(i3)
}

pub fn factorial(n: i32) -> i32 {
    if n == 2 {
        return 2;
    }

    n * factorial(n - 1)
}

pub fn fibonacci(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }

    let mut dp = Vec::with_capacity(n as usize);
    dp.push(1);
    dp.push(1);

    for i in 3..=n {
        dp.push(dp[i as usize - 2] + dp[i as usize - 3]);
    }

    dp[n as usize - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_string_works() {
        let mut string = String::from("Hello World!");
        reverse_string(&mut string);
        assert_eq!(string, "!dlroW olleH");
    }

    #[test]
    fn reverse_string_handles_multibyte_utf8() {
        let mut string = String::from("abδφ测试😂😃");
        reverse_string(&mut string);
        assert_eq!(string, "😃😂试测φδba");
    }

    #[test]
    fn max_works() {
        assert_eq!(max(3, 33, 20), 33);
    }

    #[test]
    fn factorial_works() {
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(8), 40320);
    }

    #[test]
    fn fibonacci_works() {
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
        assert_eq!(fibonacci(8), 21);
    }
}
