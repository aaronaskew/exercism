/// Yields each item of a and then each item of b
pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    struct AppendIterator<I, J>
    where
        I: Iterator,
        J: Iterator<Item = I::Item>,
    {
        first: Option<I>,
        second: Option<J>,
    }

    impl<I, J> AppendIterator<I, J>
    where
        I: Iterator,
        J: Iterator<Item = I::Item>,
    {
        fn new(first: I, second: J) -> Self {
            Self {
                first: Some(first),
                second: Some(second),
            }
        }
    }

    impl<I, J> Iterator for AppendIterator<I, J>
    where
        I: Iterator,
        J: Iterator<Item = I::Item>,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(first_iter) = &mut self.first {
                if let Some(item) = first_iter.next() {
                    return Some(item);
                }
                self.first = None;
            }

            if let Some(second_iter) = &mut self.second {
                if let Some(item) = second_iter.next() {
                    return Some(item);
                }
                self.second = None;
            }

            None
        }
    }

    AppendIterator::new(a, b)
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    struct ConcatIterator<Outer, Inner>
    where
        Outer: Iterator<Item = Inner>,
        Inner: Iterator,
    {
        outer: Outer,
        current_inner: Option<Inner>,
    }

    impl<Outer, Inner> ConcatIterator<Outer, Inner>
    where
        Outer: Iterator<Item = Inner>,
        Inner: Iterator,
    {
        fn new(nested: Outer) -> Self {
            Self {
                outer: nested,
                current_inner: None,
            }
        }
    }

    impl<Outer, Inner> Iterator for ConcatIterator<Outer, Inner>
    where
        Outer: Iterator<Item = Inner>,
        Inner: Iterator,
    {
        type Item = Inner::Item;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                if let Some(inner) = &mut self.current_inner
                    && let Some(item) = inner.next()
                {
                    return Some(item);
                }

                self.current_inner = self.outer.next();

                self.current_inner.as_ref()?;
            }
        }
    }

    ConcatIterator::new(nested_iter)
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    struct FilterIterator<I, F>
    where
        I: Iterator,
        F: Fn(&I::Item) -> bool,
    {
        iter: I,
        predicate: F,
    }

    impl<I, F> FilterIterator<I, F>
    where
        I: Iterator,
        F: Fn(&I::Item) -> bool,
    {
        fn new(iter: I, predicate: F) -> Self {
            Self { iter, predicate }
        }
    }

    impl<I, F> Iterator for FilterIterator<I, F>
    where
        I: Iterator,
        F: Fn(&I::Item) -> bool,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.by_ref().find(|item| (self.predicate)(item))
        }
    }

    FilterIterator::new(iter, predicate)
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut count = 0;

    for _ in iter {
        count += 1;
    }

    count
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    struct MapIterator<I, F, U>
    where
        I: Iterator,
        F: Fn(I::Item) -> U,
    {
        iter: I,
        function: F,
    }

    impl<I, F, U> MapIterator<I, F, U>
    where
        I: Iterator,
        F: Fn(I::Item) -> U,
    {
        fn new(iter: I, function: F) -> Self {
            Self { iter, function }
        }
    }

    impl<I, F, U> Iterator for MapIterator<I, F, U>
    where
        I: Iterator,
        F: Fn(I::Item) -> U,
    {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(item) = self.iter.next() {
                return Some((self.function)(item));
            }

            None
        }
    }

    MapIterator::new(iter, function)
}

#[allow(unused_mut)]
pub fn foldl<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut accum = initial;
    for item in iter {
        accum = (function)(accum, item);
    }

    accum
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut accum = initial;
    while let Some(item) = iter.next_back() {
        accum = (function)(accum, item);
    }

    accum
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    struct ReverseIterator<I>
    where
        I: DoubleEndedIterator,
    {
        iter: I,
    }

    impl<I> ReverseIterator<I>
    where
        I: DoubleEndedIterator,
    {
        fn new(iter: I) -> Self {
            Self { iter }
        }
    }

    impl<I> Iterator for ReverseIterator<I>
    where
        I: DoubleEndedIterator,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next_back()
        }
    }

    ReverseIterator::new(iter)
}
