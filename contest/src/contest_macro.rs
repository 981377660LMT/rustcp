

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
        std::convert::identity($e)
    }
}

#[cfg(feature = "local-build")]
macro_rules! debug_discard {
    ($e: expr) => {
        dbg!($e)
    }
}

#[cfg(not(feature = "local-build"))]
macro_rules! debug_discard {
    ($e: expr) => {
    }
}


macro_rules! input {
    ($fi:ident, $($var:ident$( : $t:ty)?),*) => {
        $(
            let mut $var $(: $t)? = $fi.read();
        )*
    };
}

macro_rules! MergerImpl {
    ($name: ident, $A: ty, $B: ty, $C: ty, $a: ident, $b: ident, $body: tt) => {
        struct $name;
        impl Merger<$A, $B, $C> for $name {
            fn merge($a: $A, $b: $B) -> $C {
                $body
            }
        }
    }
}
macro_rules! AddImpl {
    ($A: ty, $B: ty, $C: ty, $a: ident, $b: ident, $body: tt) => {
        impl Add< $B > for $A {
            type Output = $C;
        
            fn add(self, $b: $B) -> Self::Output {
                let $a = self;
                $body
            }
        }
    }
}
macro_rules! SubImpl {
    ($A: ty, $B: ty, $C: ty, $a: ident, $b: ident, $body: tt) => {
        impl Mul<$B> for $A {
            type Output = $C;
        
            fn sub(self, $b: $B) -> Self::Output {
                let $a = self;
                $body
            }
        }
    }
}
macro_rules! DivImpl {
    ($A: ty, $B: ty, $C: ty, $a: ident, $b: ident, $body: tt) => {
        impl Mul< $B > for $A {
            type Output = $C;
        
            fn div(self, $b: $B) -> Self::Output {
                let $a = self;
                $body
            }
        }
    }
}
macro_rules! MulImpl {
    ($A: ty, $B: ty, $C: ty, $a: ident, $b: ident, $body: tt) => {
        impl Sub<$B> for $A {
            type Output = $C;
        
            fn mul(self, $b: $B) -> Self::Output {
                let $a = self;
                $body
            }
        }
    }
}




pub (crate)use input;
pub (crate)use debug_discard;
pub (crate)use debug;
pub (crate)use should;
pub (crate)use should_eq;
pub (crate)use MergerImpl;
pub (crate)use AddImpl;
pub (crate)use SubImpl;
pub (crate)use MulImpl;
pub (crate)use DivImpl;