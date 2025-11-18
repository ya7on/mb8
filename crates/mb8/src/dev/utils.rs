pub fn empty_memory<const SIZE: usize>() -> Box<[u8; SIZE]> {
    vec![0; SIZE].into_boxed_slice().try_into().unwrap()
}
