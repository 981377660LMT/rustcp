use std::io;
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
        while self.offset < self.line.len() && self.line[self.offset] <= 32 {
            self.offset += 1;
        }
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
                    self.line.clear();
                    self.inner.read_until(b'\n', &mut self.line).unwrap();
                    self.offset = 0;
                }
            }
        }
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

