pub trait Load<T> {
    fn load(&mut self, arg: T);
}

pub trait Allocate<T> {
    fn allocate(&mut self, arg: T);
}

pub trait Update<T> {
    fn update(&mut self, arg: T);
}

