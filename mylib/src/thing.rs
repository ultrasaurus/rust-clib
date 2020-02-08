pub struct ThingStatus {
  pub code: i32,
  pub description: String,
}

pub trait StatusCallback {
  fn call(&self, status: ThingStatus) -> ();
} 

pub struct Thing<C: StatusCallback + Sized> {
  num: i32,
  callback: Option<C>,
}


type SimpleStatusCallback = fn(i32) -> ();
impl StatusCallback for SimpleStatusCallback {
  fn call(&self, status: ThingStatus) -> () {
    self(status.code);
  }
}

impl<C:StatusCallback + Sized> Thing<C> {
  pub fn new(num:i32, callback: Option<C>) -> Self 
  {
    Self {
      num,
      callback
    }
  }
  pub fn num(&self) -> i32 {
    self.num
  }
  pub fn something(&self) -> () {
    if let Some(callback) = &self.callback {
      callback.call(ThingStatus {
        code: self.num,
        description: "This is a string!".to_string( ),
      })  
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    static mut status_called: bool = false;

    fn status(_info: i32) -> () {
      ()
    }

    #[test]
    fn test_num() {
        let mut called = false;
        let t = Thing { num: 1, callback: Some(status as SimpleStatusCallback) };
        assert_eq!(t.num(), 1);
    }

}
