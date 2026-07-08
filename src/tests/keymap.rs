mod tests {
    use crate::Keymap;

    #[test]
    fn test_set_all_keys() {
        let mut keymap = Keymap::default();
        keymap.set_all_keys(crate::Color {
            red: 255,
            green: 35,
            blue: 45,
        });
        assert_eq!(keymap.map(), [[255, 35, 45]; 144])
    }

    #[test]
    fn test_set_key() {
        let mut keymap = Keymap::default();
        keymap.set_key(
            crate::KeymapKeys::Tab,
            crate::Color {
                red: 89,
                green: 49,
                blue: 67,
            },
        );
        let mut array = [[0, 0, 0]; 144];
        array[crate::KeymapKeys::Tab.to_usize()] = [89, 49, 67];
        assert_eq!(keymap.map(), array)
    }
}
