use super::{
    Arc, AtomicBool, AtomicU8, AtomicUsize, Entered, Mutex, Ordering, Record, RecordError,
};

pub(crate) struct Inner {
    pub(crate) after_admission: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    pub(crate) identity: usize,
    pub(crate) handles: AtomicUsize,
    pub(crate) enters: AtomicUsize,
    pub(crate) contended: AtomicBool,
    pub(crate) published: AtomicU8,
    pub(crate) last_release: AtomicU8,
}
#[derive(Clone)]
pub(crate) struct Fixture(pub(crate) Arc<Inner>);
impl Fixture {
    pub(crate) fn new() -> Self {
        Self(Arc::new(Inner {
            after_admission: Mutex::new(None),
            identity: 7,
            handles: AtomicUsize::new(1),
            enters: AtomicUsize::new(0),
            contended: AtomicBool::new(false),
            published: AtomicU8::new(0),
            last_release: AtomicU8::new(2),
        }))
    }
    pub(crate) fn release_origin(&self) {
        release(&self.0);
    }
}
fn release(inner: &Inner) {
    let previous = inner.handles.fetch_sub(1, Ordering::AcqRel);
    assert!(previous > 0);
    if previous == 1 {
        inner
            .last_release
            .store(inner.published.load(Ordering::Acquire), Ordering::Release);
    }
}
pub(crate) struct Guard<'a>(&'a Inner);
impl Drop for Guard<'_> {
    fn drop(&mut self) {
        release(self.0);
    }
}
impl Record for Fixture {
    type Context = usize;
    type Guard<'a> = Guard<'a>;
    fn enter<'a>(&'a self, context: &'a usize) -> Result<Entered<'a, Guard<'a>>, RecordError> {
        self.0.enters.fetch_add(1, Ordering::Relaxed);
        if *context != self.0.identity {
            return Err(RecordError::Foreign);
        }
        if self.0.contended.load(Ordering::Acquire) {
            return Err(RecordError::Contended);
        }
        let before = self.0.handles.load(Ordering::Acquire);
        if before == 0 {
            return Err(RecordError::Closed);
        }
        self.0
            .handles
            .compare_exchange(
                before,
                before.checked_add(1).unwrap(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map_err(|_| RecordError::Contended)?;
        let guard = Guard(&self.0);
        let callback = self.0.after_admission.lock().unwrap().take();
        if let Some(callback) = callback {
            callback();
        }
        Ok(Entered::new(guard, &self.0.published))
    }
}
