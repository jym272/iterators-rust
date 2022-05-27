//extension traits
//we defined a trait that only has the method that we want to implement(add), this trait extends the base trait Iterator
pub trait IteratorExt: Iterator {
    fn our_flatten(self) -> Flatten<Self>
    where
        Self: Sized, //we need to make sure that the type of the iterator is Sized, the compiler will not allow us to use a type that is not Sized
        Self::Item: IntoIterator, //self item implements IntoIterator
    {
        flatten(self)
    }
}
//blanket implementation for all T, where T implements Iterator
//any type that implements Iterator can be used with the extension trait, and the way to use it is to call the method flatten
impl<T: Iterator> IteratorExt for T {
    //T is the type that implements the trait: Iterator
    fn our_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator, //self item implements IntoIterator
    {
        flatten(self)
    }
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O, //iterator type of the item
    inner_current_front: Option<<O::Item as IntoIterator>::IntoIter>,
    inner_current_back: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
//any generic type must be sized
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(outer: O) -> Self {
        Flatten {
            outer,
            inner_current_front: None,
            inner_current_back: None,
        }
    }
}
impl<O> DoubleEndedIterator for Flatten<O>
where
    //must be true
    O: DoubleEndedIterator, //implies iterator
    <O as Iterator>::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            //when you take the inner iterator, take as a mutable reference
            if let Some(ref mut inner) = self.inner_current_back {
                if let Some(x) = inner.next_back() {
                    return Some(x);
                }
            }
            if let Some(inner) = self.outer.next_back() {
                self.inner_current_back = Some(inner.into_iter());
            } else {
                //1st option
                // return self
                //     .inner_current_front
                //     .as_mut()
                //     .and_then(|inner| inner.next_back());
                //2do option
                return self.inner_current_front.as_mut()?.next_back();
            }
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    <O as Iterator>::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            //as_mut: Option<T> -> Option<&mut T>
            if let Some(current) = self.inner_current_front.as_mut() {
                if let Some(item) = current.next() {
                    return Some(item);
                }
            }
            if let Some(item) = self.outer.next() {
                self.inner_current_front = Some(item.into_iter());
            } else {
                return self.inner_current_back.as_mut()?.next();
            }
        }
    }
}
//can be an iterator or a item
fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

//tests

#[cfg(test)] //compile and run the test code only when you run cargo test, not when you run cargo build.
mod test {
    use super::*;

    #[test]
    fn empty_flatten() {
        //any type that has an IntoIterator trait
        let iter = flatten(std::iter::empty::<Vec<()>>());
        assert_eq!(iter.count(), 0);
    }

    #[test]
    fn empty_wide() {
        let iter = flatten(vec![Vec::<()>::new(), vec![], vec![]]);
        assert_eq!(iter.count(), 0);
    }

    #[test]
    fn one_item() {
        let iter = flatten(vec![vec![()]]);
        let iter_ = flatten(std::iter::once(vec![()]));

        assert_eq!(iter.count(), 1);
        assert_eq!(iter_.count(), 1);
    }

    #[test]
    fn two_items() {
        let iter = flatten(vec![vec![()], vec![()]]);
        let iter_ = flatten(std::iter::once(vec!["a", "b", "c"]));

        assert_eq!(iter.count(), 2);
        assert_eq!(iter_.count(), 3);
    }
    #[test]
    fn reverse_wide() {
        assert_eq!(flatten(vec![vec![()], vec![()]]).rev().count(), 2);
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn test_flatten() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut iter = flatten(v);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn reverse_test_flatten() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut iter = flatten(v);
        assert_eq!(iter.next_back(), Some(9));
        assert_eq!(iter.next_back(), Some(8));
        assert_eq!(iter.next_back(), Some(7));
        assert_eq!(iter.next_back(), Some(6));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), None);
    }
    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3")); //a3 instead of b3
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
    #[test]
    fn both_ends_first_back_bug() {
        let mut iter = vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]
            .into_iter()
            .our_flatten();
        assert_eq!(iter.next_back(), Some("b3")); //a3 instead of b3
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next(), Some("b1"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
    #[test]
    fn both_ends_first_front_bug() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next_back(), Some("a3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
    #[test]
    fn inf() {
        //this inf iterator in not double ended
        let mut iter = flatten((0..).map(|i| 0..i));
        // 0 => 0..0 => empty
        // 1 => 0..1 => [0]
        // 2 => 0..2 => [0, 1]
        // 3 => 0..3 => [0, 1, 2]
        // 4 => 0..4 => [0, 1, 2, 3]
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
    }
    #[test]
    fn deep() {
        let count = vec![vec![vec![1, 2, 3], vec![4, 5, 6]], vec![vec![7, 8, 9]]]
            .into_iter()
            .our_flatten()
            .our_flatten()
            .count();
        assert_eq!(count, 9);
    }
}
