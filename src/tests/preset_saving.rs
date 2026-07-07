mod tests {
    use std::collections::VecDeque;

    use crate::Keymap;
    #[allow(unused_imports)]
    use crate::Preset;

    #[test]
    fn test_encode_decode() {
        let preset = Preset::new(
            crate::Effects::Static,
            crate::Color {
                red: 0,
                green: 0,
                blue: 0,
            },
            true,
            6,
            1,
            0,
            None,
        );
        let mut buffer: VecDeque<u8> = preset._encode_to_buffer().into();
        assert_eq!(
            preset,
            Preset::_decode_from_buffer(&mut buffer).expect("error decoding")
        );

        let preset = Preset::new(
            crate::Effects::VerticalWave,
            crate::Color {
                red: 0,
                green: 0,
                blue: 0,
            },
            true,
            2,
            9,
            2,
            None,
        );
        let mut buffer: VecDeque<u8> = preset._encode_to_buffer().into();
        assert_eq!(
            preset,
            Preset::_decode_from_buffer(&mut buffer).expect("error decoding")
        );

        let preset = Preset::new(
            crate::Effects::HorizontalWave,
            crate::Color {
                red: 67,
                green: 69,
                blue: 255,
            },
            false,
            2,
            9,
            1,
            None,
        );
        let mut buffer: VecDeque<u8> = preset._encode_to_buffer().into();
        assert_eq!(
            preset,
            Preset::_decode_from_buffer(&mut buffer).expect("error decoding")
        );

        let mut keymap: Keymap = Default::default();
        keymap.set_all_keys(crate::Color {
            red: 255,
            green: 9,
            blue: 7,
        });
        keymap.set_key(
            crate::KeymapKeys::C,
            crate::Color {
                red: 111,
                green: 222,
                blue: 133,
            },
        );
        keymap.set_key(
            crate::KeymapKeys::NumPlus,
            crate::Color {
                red: 112,
                green: 223,
                blue: 134,
            },
        );
        let preset = Preset::new(
            crate::Effects::StaticPerKey,
            crate::Color {
                red: 0,
                green: 0,
                blue: 0,
            },
            false,
            16,
            1,
            0,
            Some(keymap.map()),
        );
        let mut buffer: VecDeque<u8> = preset._encode_to_buffer().into();
        assert_eq!(
            preset,
            Preset::_decode_from_buffer(&mut buffer).expect("error decoding")
        )
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_save_load() {
        use std::fs::File;

        let preset = Preset::new(
            crate::Effects::Static,
            crate::Color {
                red: 0,
                green: 0,
                blue: 0,
            },
            true,
            6,
            1,
            0,
            None,
        );
        preset
            .save_to_file(&mut File::create("/tmp/test_file.gk1k").expect("Failed to create file"))
            .expect("failed to save");
        assert_eq!(
            preset,
            Preset::load_from_file(
                &mut File::open("/tmp/test_file.gk1k").expect("failed to open file")
            )
            .expect("error loading from file")
        );

        let preset = Preset::new(
            crate::Effects::VerticalWave,
            crate::Color {
                red: 0,
                green: 0,
                blue: 0,
            },
            true,
            2,
            9,
            2,
            None,
        );
        preset
            .save_to_file(&mut File::create("/tmp/test_file.gk1k").expect("Failed to create file"))
            .expect("failed to save");
        assert_eq!(
            preset,
            Preset::load_from_file(
                &mut File::open("/tmp/test_file.gk1k").expect("failed to open file")
            )
            .expect("error loading from file")
        );

        let preset = Preset::new(
            crate::Effects::HorizontalWave,
            crate::Color {
                red: 67,
                green: 69,
                blue: 255,
            },
            false,
            2,
            9,
            1,
            None,
        );
        preset
            .save_to_file(&mut File::create("/tmp/test_file.gk1k").expect("Failed to create file"))
            .expect("failed to save");
        assert_eq!(
            preset,
            Preset::load_from_file(
                &mut File::open("/tmp/test_file.gk1k").expect("failed to open file")
            )
            .expect("error loading from file")
        );

        let mut keymap: Keymap = Default::default();
        keymap.set_all_keys(crate::Color {
            red: 255,
            green: 9,
            blue: 7,
        });
        keymap.set_key(
            crate::KeymapKeys::C,
            crate::Color {
                red: 111,
                green: 222,
                blue: 133,
            },
        );
        keymap.set_key(
            crate::KeymapKeys::NumPlus,
            crate::Color {
                red: 112,
                green: 223,
                blue: 134,
            },
        );
        let preset = Preset::new(
            crate::Effects::StaticPerKey,
            crate::Color {
                red: 0,
                green: 0,
                blue: 0,
            },
            false,
            16,
            1,
            0,
            Some(keymap.map()),
        );
        preset
            .save_to_file(&mut File::create("/tmp/test_file.gk1k").expect("Failed to create file"))
            .expect("failed to save");
        assert_eq!(
            preset,
            Preset::load_from_file(
                &mut File::open("/tmp/test_file.gk1k").expect("failed to open file")
            )
            .expect("error loading from file")
        );
    }
}
