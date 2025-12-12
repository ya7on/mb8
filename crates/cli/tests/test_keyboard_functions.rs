#[cfg(test)]
mod tests {
    use mb8_cli::keyboard::Keyboard;
    use minifb::Key;

    #[test]
    fn test_map_key_to_char_basic() {
        //Testing key map to char
        assert_eq!(Keyboard::map_key_to_char(Key::A, false), Some(b'a'));
        assert_eq!(Keyboard::map_key_to_char(Key::A, true), Some(b'A'));
        assert_eq!(Keyboard::map_key_to_char(Key::Space, false), Some(b' '));
        assert_eq!(Keyboard::map_key_to_char(Key::Enter, false), Some(b'\n'));
    }

    #[test]
    fn test_map_key_to_char_numbers() {
        //Testing Key map to numbers
        assert_eq!(Keyboard::map_key_to_char(Key::Key0, false), Some(b'0'));
        assert_eq!(Keyboard::map_key_to_char(Key::Key9, false), Some(b'9'));
    }
}
