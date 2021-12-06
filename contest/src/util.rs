#[cfg(feature = "local-build")]
macro_rules! should {
    ($($e: expr),*) => {
        $(
            assert!($e);
        )*
    }
}

#[cfg(not(feature = "local-build"))]
macro_rules! should {
    ($($e: expr),*) => {
    }
}

#[cfg(feature = "local-build")]
macro_rules! should_eq {
    ($($a: expr, $b: expr);*) => {
        $(
            assert_eq!($a, $b);
        )*
    }
}

#[cfg(not(feature = "local-build"))]
macro_rules! should_eq {
    ($($e: expr),*) => {
    }
}



#[cfg(feature = "local-build")]
macro_rules! debug {
    ($e: expr) => {
        dbg!($e)
    }
}

#[cfg(not(feature = "local-build"))]
macro_rules! debug {
    ($e: expr) => {
        $e
    }
}

pub (crate)use debug;
pub (crate)use should;
pub (crate)use should_eq;