pub trait Lens<T>: Clone + Send + Sync + 'static {
    type Source: 'static;

    fn lens<R>(&self, value: &Self::Source, f: impl FnOnce(&T) -> R) -> R;
    fn lens_mut<R>(&self, value: &mut Self::Source, f: impl FnOnce(&mut T) -> R) -> R;

    fn wrap<B>(&self, wrapper: B) -> Product<B, Self> where B: Lens<Self::Source>,  {
        Product(wrapper, self.clone())
    }
}

#[derive(Copy, Clone)]
pub struct Empty;

impl<T: 'static> Lens<T> for Empty {
    type Source = T;

    fn lens<R>(&self, value: &T, f: impl FnOnce(&T) -> R) -> R {
        f(value)
    }

    fn lens_mut<R>(&self, value: &mut T, f: impl FnOnce(&mut T) -> R) -> R {
        f(value)
    }
}

#[derive(Clone)]
pub struct Product<A: Clone, B: Clone>(pub A, pub B);

impl<A, B, T> Lens<T> for Product<A, B> where A: Lens<B::Source>, B: Lens<T> {
    type Source = A::Source;

    fn lens<R>(&self, value: &Self::Source, f: impl FnOnce(&T) -> R) -> R {
        self.0.lens(value, |u|self.1.lens(u, f))
    }

    fn lens_mut<R>(&self, value: &mut Self::Source, f: impl FnOnce(&mut T) -> R) -> R {
        self.0.lens_mut(value, |u|self.1.lens_mut(u, f))
    }
}