#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(clippy::all)]
mod sys {
    // to make clippy happy
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use sys::*;

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        
    }

}