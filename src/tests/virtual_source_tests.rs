
    use crate::source_file::VirtualSourceFile;

    #[test]
    fn add_line() {
        let file = VirtualSourceFile::new("imaginary-test-file".to_string());

        let one = file.add_line("one");
        let two = file.add_line("two");
        let three = file.add_line("three");

        assert_eq!(one.data, "one");
        assert_eq!(two.data, "two");
        assert_eq!(three.data, "three");
    }

    #[test]
    fn line_containing() {
        let file = VirtualSourceFile::new("imaginary-test-file".to_string());

        file.add_line("one");
        let two = file.add_line("two");
        file.add_line("three");

        let (line_str, pos) = file.line_containing(two.data).expect("Must find line");

        assert_eq!(line_str, "two");
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 1);
    }

