struct xorshift128p_state(u64, u64);

#[cfg(feature = "local-build")]
fn seed() -> u64 {
    dbg!("use fix seed 0");
    0
}

#[cfg(not(feature = "local-build"))]
fn seed() -> u64 {
    let start = std::time::SystemTime::now();
    start
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

fn splitmix64(state: &mut u64) -> u64 {
    *state += 0x9E3779B97f4A7C15;
    let mut result = *state;
    result = (result ^ (result >> 30)) * 0xBF58476D1CE4E5B9;
    result = (result ^ (result >> 27)) * 0x94D049BB133111EB;
    return result ^ (result >> 31);
}

fn xorshift128p_init(seed: u64) -> xorshift128p_state {
    let mut smstate = seed;
    let mut result = xorshift128p_state(0, 0);

    result.0 = splitmix64(&mut smstate);
    result.1 = splitmix64(&mut smstate);

    result
}

fn xorshift128p(state: &mut xorshift128p_state) -> u64 {
    let mut t = state.0;
    let s = state.1;
    state.0 = s;
    t ^= t << 32;
    t ^= t >> 18;
    t ^= s ^ (s >> 5);
    state.1 = t;
    t + s
}

pub fn rng() -> &'static mut Rng {
    static mut singleton: Option<Rng> = None;
    unsafe {
        match &mut singleton {
            Some(x) => x,
            None => {
                singleton = Some(Rng::new());
                rng()
            }
        }
    }
}

pub struct Rng {
    state: xorshift128p_state,
}

impl Rng {
    pub fn new() -> Self {
        Self {
            state: xorshift128p_init(seed()),
        }
    }
    pub fn init(&mut self, seed: u64) {
        self.state = xorshift128p_init(seed);
    }
    pub fn new_with_seed(seed: u64) -> Self {
        Self {
            state: xorshift128p_init(seed)
        }
    }

    pub fn u64(&mut self) -> u64 {
        xorshift128p(&mut self.state)
    }
    pub fn usize(&mut self) -> usize {
        self.u64() as usize
    }
    pub fn f64(&mut self) -> f64 {
        loop {
            let res = self.u64();
            if res < u64::MAX {
                return res as f64 / usize::MAX as f64;
            }
        }
    }
    pub fn limit_usize(&mut self, n: usize) -> usize {
        self.limit_u64(n as u64) as usize
    }
    pub fn limit_u64(&mut self, n: u64) -> u64 {
        (self.f64() * n as f64) as u64
    }
    pub fn range_u64(&mut self, l: u64, r: u64) -> u64 {
        let n = r - l + 1;
        self.limit_u64(r - l + 1) + l
    }
    pub fn range_usize(&mut self, l: usize, r: usize) -> usize {
        self.range_u64(l as u64, r as u64) as usize
    }
}
