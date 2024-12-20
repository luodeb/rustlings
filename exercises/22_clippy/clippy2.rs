fn main() {
    let mut res = 42;
    let num = Some(12);
    // TODO: Fix the Clippy lint.
    if let Some(x) = num {
        res += x;
    }

    println!("{res}");
}
