use crate::CallStack;

struct Thread {
    call_stack: CallStack,
}

impl Thread {
    fn new() -> Self {
        Thread {
            call_stack: CallStack::new(),
        }
    }
}
