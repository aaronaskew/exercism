use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T>
where
    T: ToString + Clone,
{
    function: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T>
where
    T: ToString + Clone,
{
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString,
    {
        Self {
            function: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T>
where
    T: ToString + Clone,
{
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: ToString + Clone,
{
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);

        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |item| {
            let mut new_text = String::new();

            for matcher in &self.matchers {
                if (matcher.function)(item.clone()) {
                    new_text += &matcher.subs;
                }
            }

            if new_text.is_empty() {
                new_text = item.to_string();
            }

            new_text
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: ToString + Clone + Rem<Output = T> + PartialEq + From<u8>,
{
    let matchers = vec![
        Matcher::new(|x| x % 3.into() == 0.into(), "fizz"),
        Matcher::new(|x| x % 5.into() == 0.into(), "buzz"),
    ];

    Fizzy { matchers }
}
