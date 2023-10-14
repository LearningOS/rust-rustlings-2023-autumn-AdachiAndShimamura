// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

/// # Safety
///
/// The `address` must contain a valid memory address pointing to a mutable `u32` value.
/// This function reads the value from the given address, modifies it, and writes it back.
///
/// It is the caller's responsibility to ensure that the `address` is a valid pointer and that
/// the pointer points to a valid `u32` value. Additionally, the caller should make sure that
/// this function does not cause data races by ensuring exclusive access to the memory location.
///
/// This function is unsafe because it operates with raw pointers and has no way to guarantee
/// memory safety or aliasing rules at compile time.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: We assume that `address` points to a valid mutable `u32`.
    // This is safe if and only if the caller ensures the safety requirements mentioned in the
    // safety documentation are met.
    let ptr = address as *mut u32;
    let value = *ptr; // Read the value.
    // Modify the value in some way.
    let modified_value = 0xAABBCCDD;
    *ptr = modified_value; // Write the modified value back.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
