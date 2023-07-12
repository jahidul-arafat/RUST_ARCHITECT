pub fn challenge(){
    let msg = "  Hello, world! \t  test 🔱      ";
    println!("{}, {}", msg, msg.len());
    // let trimed_msg = msg.trim();
    // println!("{}, {}", trimed_msg, trimed_msg.len());
    let trimmed_msg = remove_spaces(&msg);
    println!("{}, {}", trimmed_msg, trimmed_msg.len());

}

/*
Certainly! Let's simulate the step-by-step execution of the `remove_spaces` function for the input `'     welcome.  !      '`:

1. The input string is `'     welcome.  !      '`.
2. The byte representation of the input string is `[' ', ' ', ' ', ' ', ' ', 'w', 'e', 'l', 'c', 'o', 'm', 'e', '.', ' ', ' ', '!', ' ', ' ', ' ', ' ', ' ', ' ']`.
3. The `start` and `end` variables are initially set to `0` and `22`, respectively, representing the start and end indices of the non-space portion of the string.
4. The first loop iterates through the input string from left to right to find the index of the first non-space character. In this case, it increments `start` five times until it reaches the character `'w'` at index `5`.
5. The second loop iterates through the input string from right to left to find the index of the last non-space character. In this case, it decrements `end` three times until it reaches the character `'!'` at index `16`.
6. After the loops complete, `start` is `5` and `end` is `16`.
7. The resulting string slice is obtained using `&input[start..end]`, which corresponds to the substring `'welcome.  !'`.
8. The resulting string slice is returned by the function.
9. The final output is printed: `Result: 'welcome.  !'`.

As a result, the function removes the leading and trailing spaces from the input string while preserving the internal spaces and other characters.
 */

// method to remove leading and trailing spaces from a string
fn remove_spaces(input: &str) -> &str {
    let mut bytes = input.as_bytes();
    let mut start = 0;
    let mut end = bytes.len();

    // Find the index of the first non-space character
    while start < end && bytes[start].is_ascii_whitespace() {
        start += 1;
    }

    // Find the index of the last non-space character
    while start < end && bytes[end-1].is_ascii_whitespace() {
        end -= 1;
    }

    &input[start..end]
}

