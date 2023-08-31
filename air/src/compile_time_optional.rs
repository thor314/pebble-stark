// Assuming that N is passed in some way to your struct.
trait CompileTimeOptional<T, const N: usize> {
    type CompileType;
}

// Specialization for when the condition is met (N > bound).
impl<T, const N: usize, const BOUND: usize> CompileTimeOptional<T, N> for std::marker::PhantomData<T>
where
    [(); N - BOUND - 1]: ,
{
    type CompileType = T;
}

// Specialization for when the condition is not met (N <= bound).
impl<T, const N: usize, const BOUND: usize> CompileTimeOptional<T, N> for std::marker::PhantomData<[(); BOUND - N]>
{
    type CompileType = HiddenMember<T>;
}

struct HiddenMember<T>(T);

impl<T> HiddenMember<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn extract_value(&self) -> &T {
        &self.0
    }
}