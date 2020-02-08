mod thingc;
mod thing;
use thing::Thing as Thing;

#[no_mangle]
pub extern "C" fn add(a: i32, b:i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn add_positive_numbers() {
        assert_eq!(add(2,2), 4);
    }

    #[test]
    fn add_negative_numbers() {
        assert_eq!(add(-2,-3), -5);
    }

}
