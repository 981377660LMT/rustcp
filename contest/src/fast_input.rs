use std::io::{self, BufRead, BufReader, Cursor};
///
/// Fast input for competitive programming
///
/// # Unsafe
///
/// Only support ascii
///
/// # Example
///
/// ```not_run
/// let mut stdin = io::stdin();
/// let mut fi = FastInput::new(stdin.lock());
/// ```
///
///
pub struct FastInput<R: std::io::BufRead> {
    inner: R,
    line: Vec<u8>,
    offset: usize,
}

pub fn new_fastinput_from_string(s: String) -> FastInput<BufReader<Cursor<String>>> {
    let x = FastInput::new(BufReader::new(Cursor::new(s)));
    x
}

pub fn new_fastinput_from_string_ref(s: &String) -> FastInput<BufReader<Cursor<&String>>> {
    let x = FastInput::new(BufReader::new(Cursor::new(s)));
    x
}

impl<R: std::io::BufRead> FastInput<R> {
    pub fn new(inner: R) -> Self {
        FastInput {
            inner,
            line: Vec::with_capacity(1 << 15),
            offset: 0,
        }
    }

    ///
    /// Read next token of current processed line
    ///
    fn next(&mut self) -> Option<&str> {
        self.skip_blank();
        if self.offset == self.line.len() {
            return None;
        }
        let begin = self.offset;
        let mut end = self.offset + 1;
        while end < self.line.len() && self.line[end] > 32 {
            end += 1;
        }
        self.offset = end;
        unsafe { Some(std::str::from_utf8_unchecked(&self.line[begin..end])) }
    }

    fn skip_blank(&mut self) {
        while self.offset < self.line.len() && self.line[self.offset] <= 32 {
            self.offset += 1;
        }
    }

    pub fn eof(&mut self) -> bool {
        loop {
            self.skip_blank();
            if self.offset < self.line.len() {
                return false;
            }
            if !self.refill() {
                return true;
            }
        }
    }

    ///
    /// Read a specified type element from input
    ///
    /// # Unsafe
    ///
    /// This method might enter infinite loop if no more input available
    ///
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        loop {
            match self.next() {
                Some(token) => {
                    return token.parse().ok().expect("Wrong format input");
                }
                None => {
                    self.refill();
                }
            }
        }
    }

    ///
    /// return false if the stream reached eof 
    /// 
    fn refill(&mut self) -> bool {
        self.line.clear();
        let num = self.inner.read_until(b'\n', &mut self.line).unwrap();
        self.offset = 0;
        return num > 0;
    }

    pub fn r<T: std::str::FromStr>(&mut self) -> T {
        self.read()
    }

    pub fn ri(&mut self) -> i32 {
        let res: i32 = self.read();
        return res;
    }

    pub fn rl(&mut self) -> i64 {
        let res: i64 = self.read();
        return res;
    }

    pub fn rf(&mut self) -> f64 {
        let res: f64 = self.read();
        return res;
    }

    pub fn rs(&mut self) -> String {
        let res: String = self.read();
        return res;
    }

    pub fn ru(&mut self) -> usize {
        let res: usize = self.read();
        return res;
    }
}
