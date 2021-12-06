use std::io::BufWriter;
use contest::{fast_input::FastInput, solver::solve_multi};


fn main() {
    unsafe {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut fi = FastInput::new(stdin.lock());
        let mut fo = BufWriter::new(stdout.lock());
        solve_multi(&mut fi, &mut fo);
    }
}
