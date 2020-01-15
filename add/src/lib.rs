
#[no_mangle]
pub extern "C" fn add(a: u32, b:u32) -> u32 {
    a + b
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::add;
        assert_eq!(add(2,2), 4);
    }
}
