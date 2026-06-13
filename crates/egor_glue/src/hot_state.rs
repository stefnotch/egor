use dioxus_devtools::subsecond;

pub struct HotState<T> {
    state: Box<T>,
    state_ptr: subsecond::HotFnPtr,
}

impl<T: 'static> HotState<T> {
    pub fn new<F: FnMut() -> Box<T>>(create_state: F) -> Self {
        let mut state_fn = subsecond::HotFn::current(create_state);
        let state_ptr = state_fn.ptr_address();
        let state = state_fn.call(());
        Self { state, state_ptr }
    }

    pub fn get<F: FnMut() -> Box<T>>(&mut self, create_state: F) -> &mut T {
        let mut state_fn = subsecond::HotFn::current(create_state);
        let state_ptr = state_fn.ptr_address();
        if state_ptr != self.state_ptr {
            self.state = state_fn.call(());
            self.state_ptr = state_ptr;
        }
        self.state.as_mut()
    }
}
