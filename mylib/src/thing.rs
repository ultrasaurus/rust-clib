
pub struct Thing {
  num: i32,
}

impl Thing {
  pub fn new(num:i32) -> Self {
    Self {
      num
    }
  }
  pub fn num(&self) -> i32 {
    self.num
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num() {
        let t = Thing { num: 1 };
        assert_eq!(t.num(), 1);
    }

}
