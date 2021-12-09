use std::{io::BufWriter, thread};
use contest::{fast_input::FastInput, solver::solve_multi};

unsafe fn run_in_current_thread() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut fi = FastInput::new(stdin.lock());
    let mut fo = BufWriter::new(stdout.lock());
    solve_multi(&mut fi, &mut fo);
}
unsafe fn run_in_new_thread() {
    thread::Builder::new()
    .stack_size(256 << 20)
    .spawn(|| {run_in_current_thread();})
    .unwrap()
    .join();
}

fn main() {
    unsafe {
        run_in_current_thread();
    }
}
