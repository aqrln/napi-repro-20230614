use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[napi]
pub fn s_to_e(s: Struct) -> Enum {
    if s.value > 0 {
        Enum::A
    } else {
        Enum::B
    }
}

#[napi]
pub fn e_to_s(e: Enum) -> Struct {
    Struct {
        value: match e {
            Enum::A => 1,
            Enum::B => -1,
        },
    }
}

#[napi(object)]
#[derive(Debug, PartialEq)]
pub struct Struct {
    pub value: i32,
}

#[napi]
#[derive(Debug, PartialEq)]
pub enum Enum {
    A,
    B,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn s_to_e_works() {
        let result = s_to_e(Struct { value: 10 });
        assert_eq!(result, Enum::A);
    }

    #[test]
    fn e_to_s_works() {
        let result = e_to_s(Enum::A);
        assert_eq!(result, Struct { value: 1 });
    }
}
