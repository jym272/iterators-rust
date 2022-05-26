struct Flatten<O> {
    outer: O,
}

impl<O> Flatten<O> {
    fn new(iter: O) -> Flatten<O> {
        Flatten {
            outer: iter,
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
        self.outer.next().and_then(|item| item.into_iter().next())
    }
}

fn flatten<I>(iter: I) -> Flatten<I> {
    Flatten::new(iter)
}


//tests

#[cfg(test)] //compile and run the test code only when you run cargo test, not when you run cargo build.
mod test{
    use super::*;

    #[test]
    fn empty_flatten() {                                                  //any type that has an IntoIterator trait
        let  iter = flatten(std::iter::empty::<Vec<()>>());
        assert_eq!(iter.count(),0);
    }
    #[test]
    fn empty_wide(){
        let iter = flatten(vec![Vec::<()>::new(),vec![],vec![]].into_iter());
        assert_eq!(iter.count(),0);
    }

    #[test]
    fn one_item(){
        let iter = flatten(vec![vec![()]].into_iter());
        let iter_ = flatten(std::iter::once(vec![()]));

        assert_eq!(iter.count(),1);
        assert_eq!(iter_.count(),1);
    }
    #[test]
    fn two_items(){
        let iter = flatten(vec![vec![()],vec![()]].into_iter());
        let iter_ = flatten(std::iter::once(vec!["a","a"]));

        assert_eq!(iter.count(),2);
        assert_eq!(iter_.count(),2);
    }

    // #[test]
    fn test_flatten(){
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut iter = flatten(v.into_iter());
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
}