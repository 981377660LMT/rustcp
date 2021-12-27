use std::{io::{BufRead, Write}, mem::swap};

use template::{fast_input::FastInput, rand::Rng};


pub unsafe fn brute_force<InT>(fi: &mut FastInput<InT>, fo: &mut impl Write)
where
    InT: BufRead{
 
}

pub fn generate_test(rng: &mut Rng, fo: &mut impl Write) {

}

pub fn against(a: &String, b: &String) -> bool {
    let da: Vec<_> = a.split_ascii_whitespace().collect();    
    let db: Vec<_> = b.split_ascii_whitespace().collect();
    da == db
}