pub struct State {
  counter: i32
}

impl State {
  fn increment(&mut self) {
      self.counter = self.counter + 1;
  }

  fn decrement(&mut self) {
      self.counter = self.counter - 1;
  }
}

#[no_mangle]
pub unsafe fn demo_increment(state: *mut State) -> i32 {
    let state = &mut *state;
    state.increment();
    state.counter
}

#[no_mangle]
pub unsafe fn demo_decrement(state: *mut State) -> i32 {
    let state = &mut *state;
    state.decrement();
    state.counter
}

#[no_mangle]
pub fn demo_new_state() -> *mut State {
    Box::into_raw(Box::new(State { counter: 0 }))
}


#[cfg(test)]
mod tests {
    use State;

    #[test]
    fn it_increments() {
        let state = State { counter: 0 };
        let new_state = state.increment();
        assert_eq!(new_state.counter, 1);
    }

    #[test]
    fn it_increments_twice() {
        let state = State { counter: 0 };
        let new_state = state.increment();
        let new_new_state = new_state.increment();
        assert_eq!(new_new_state.counter, 2);
    }

    #[test]
    fn it_decrements() {
        let state = State { counter: 1 };
        let new_state = state.decrement();
        assert_eq!(new_state.counter, 0);
    }

    #[test]
    fn it_decrements_twice() {
        let state = State { counter: 2 };
        let new_state = state.decrement();
        let new_new_state = new_state.decrement();
        assert_eq!(new_new_state.counter, 0);
    }

    #[test]
    fn it_decrements_below_zero() {
        let state = State { counter: 0 };
        let new_state = state.decrement();
        assert_eq!(new_state.counter, -1);
    }
}
