pub trait Load<T> {
    fn load(&mut self, arg: T);
}

pub trait Allocate<T> {
    fn allocate(&mut self, arg: T) -> &mut Self;
}

pub trait Update<T> {
    fn update(&mut self, arg: T);
}

pub trait Draw<T> {
    fn draw(&mut self, arg: T);
}
