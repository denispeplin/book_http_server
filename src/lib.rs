pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(_size: usize) -> Self {
        ThreadPool {}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        f();
    }
}
