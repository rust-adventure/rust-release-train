fn main() {
    // 1.74: non-exhaustive patterns: `_` not covered
    // 1.75: works.
    match 0usize {
        0.. => {} // exhaustive!
    }
}
