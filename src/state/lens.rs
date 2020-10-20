pub trait Lens<T>: Clone + Send + Sync + 'static {
    type Source: 'static;

    fn with<R>(&self, value: &Self::Source, f: impl FnOnce(&T) -> R) -> R;
    fn with_mut<R>(&self, value: &mut Self::Source, f: impl FnOnce(&mut T) -> R) -> R;

    fn wrap<B>(&self, wrapper: B) -> Then<B, Self> where B: Lens<Self::Source>,  {
        Then(wrapper, self.clone())
    }
}

#[derive(Copy, Clone)]
pub struct Id;

impl<T: 'static> Lens<T> for Id {
    type Source = T;

    fn with<R>(&self, value: &T, f: impl FnOnce(&T) -> R) -> R {
        f(value)
    }

    fn with_mut<R>(&self, value: &mut T, f: impl FnOnce(&mut T) -> R) -> R {
        f(value)
    }
}

#[derive(Clone)]
pub struct Then<A: Clone, B: Clone>(pub A, pub B);

impl<A, B, T> Lens<T> for Then<A, B> where A: Lens<B::Source>, B: Lens<T> {
    type Source = A::Source;

    fn with<R>(&self, value: &Self::Source, f: impl FnOnce(&T) -> R) -> R {
        self.0.with(value, |u|self.1.with(u, f))
    }

    fn with_mut<R>(&self, value: &mut Self::Source, f: impl FnOnce(&mut T) -> R) -> R {
        self.0.with_mut(value, |u|self.1.with_mut(u, f))
    }
}