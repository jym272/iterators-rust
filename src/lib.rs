struct Flatten<O> where O: Iterator, O::Item: IntoIterator
{
    outer: O,
    inner_current: Option<<O::Item as IntoIterator>::IntoIter>,
    inner_current_back: Option<<O::Item as IntoIterator>::IntoIter>,


}

impl<O> Flatten<O> where O: Iterator, O::Item: IntoIterator
{
    fn new(outer: O) -> Self {
        Flatten {
            outer,
            inner_current: None,
            inner_current_back: None,
        }
    }
}
impl<O> DoubleEndedIterator for Flatten<O>
    where  //must be true
        O: DoubleEndedIterator, //implies iterator
        <O as Iterator>::Item: IntoIterator,
        <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.inner_current_back {
                if let Some(x) = inner.next_back() {
                    return Some(x);
                }
            }
            if let Some(x) = self.outer.next_back() {
                self.inner_current_back = Some(x.into_iter());
            } else {
                return None;
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
            if let Some(ref mut current) = self.inner_current {
                if let Some(item) = current.next() {
                    return Some(item);
                }
            }
            //2nd Option:
            let next = self.outer.next()?;
            self.inner_current = Some(next.into_iter());

            //1st Option:
            // if let Some(item) = self.outer.next() {
            //     self.inner_current = Some(item.into_iter());
            // } else {
            //     return None;
            // }
        }
    }
}
                //can be an iterator or a item
fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
    where
        I: IntoIterator,
        I::Item: IntoIterator
{
    Flatten::new(iter.into_iter())
}


//tests

#[cfg(test)] //compile and run the test code only when you run cargo test, not when you run cargo build.
mod test {
    use super::*;

    #[test]
    fn empty_flatten() {                                                  //any type that has an IntoIterator trait
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
        let iter_ = flatten(std::iter::once(vec!["a","b","c"]));

        assert_eq!(iter.count(), 2);
        assert_eq!(iter_.count(), 3);
    }
    #[test]
    fn reverse_wide(){
        assert_eq!(flatten(vec![vec![()], vec![()]]).rev().count(), 2);
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).rev().collect::<Vec<_>>(), vec!["b", "a"]);
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
    fn both_ends(){
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


}