// thinking...
// - State implements functions.
// - Actions are strings that map to functions that State implements.
// - Actions are dispatched in response to user interactions in JS, updating
//   state in rust. unsafe?
// pub fn update -> something something
//
pub struct State {
  counter: i32
}

impl State {
  fn increment(&self) -> State {
    State { counter: &self.counter + 1 }
  }

  fn decrement(&self) -> State {
    State { counter: &self.counter - 1 }
  }
}

static mut STATE: State = State { counter: 0 };

#[no_mangle]
pub fn demo_increment() -> i32 {
    unsafe {
        STATE = STATE.increment();
        STATE.counter
    }
}

#[no_mangle]
pub fn demo_decrement() -> i32 {
    unsafe {
        STATE = STATE.decrement();
        STATE.counter
    }
}

fn main() {

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
