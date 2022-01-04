use std::{io::{Write, BufReader, Cursor}, fmt::Display};

use crate::{fast_input::FastInput, rand::{Rng, rng}};

use crate::{solver::solve_multi, stress_external_member::{generate_test, against, brute_force}};

fn printable_input(s: String) -> String {
    let len = 1000;
    if s.len() < len {
        s
    } else {
        s[0..len].to_string() + "..."
    }
}

pub unsafe fn stress() {
    let mut round = 0;
    let mut rng_for_input_generator = Rng::new_with_seed(0);
    loop {
        round += 1;
        if round >= 10000 {
            println!("Pass stress!");
            return;
        }
        //init seed for program
        rng().init(0);

        let mut input = OutputWrapper::new();
        generate_test(&mut rng_for_input_generator, &mut input);
        println!("Test {}:", round);
        println!("{}\n", printable_input(input.to_string()));
        let mut actual_output = OutputWrapper::new();
        let mut expect_output = OutputWrapper::new();
        solve_multi(&mut FastInput::new(BufReader::new(Cursor::new(input.to_string()))), &mut actual_output);
        brute_force(&mut FastInput::new(BufReader::new(Cursor::new(input.to_string()))), &mut expect_output);
        if !against(&actual_output.to_string(), &expect_output.to_string()) {
            println!("Test case {}:", round);
            println!("\nInput:\n{}", input);
            println!("\nExpect:\n{}", expect_output);
            println!("\nActual:\n{}", actual_output);
            panic!("Fail!");
        }
    }
}
struct OutputWrapper(Vec<u8>);
impl OutputWrapper {
    pub fn new() -> OutputWrapper {
        OutputWrapper(Vec::new())
    }
}
impl Display for OutputWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8(self.0.clone()).unwrap();
        f.write_str(s.as_str())
    }
}
impl Write for OutputWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut to = buf.iter().map(|x| *x).collect();
        self.0.append(&mut to);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}