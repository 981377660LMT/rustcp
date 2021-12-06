#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Read};

    use crate::fast_input::*;

    #[test]
    fn test_read() {
        let s = "123\n-4 -5\r abcd\n 1.5";
        let mut c = Cursor::new(s);
        let mut reader = BufReader::new(c);
        let mut reader = FastInput::new(reader);
        assert_eq!(123, reader.read());
        assert_eq!(-4, reader.read());
        assert_eq!(-5, reader.read());
        let x: String = reader.read();
        assert_eq!("abcd".to_string(), x);
        assert_eq!(1.5, reader.read());
    }
}
