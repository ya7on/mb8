#[must_use]
/// Creates a new empty memory block of size `SIZE`.
///
/// # Panics
///
/// This function will panic if the size of the memory block is not a multiple of the page size.
pub fn empty_memory<const SIZE: usize>() -> Box<[u8; SIZE]> {
    #[allow(clippy::unwrap_used)]
    vec![0; SIZE].into_boxed_slice().try_into().unwrap()
}
