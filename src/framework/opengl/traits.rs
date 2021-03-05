pub trait Load<T> {
    fn load(&mut self, arg: T);
}
