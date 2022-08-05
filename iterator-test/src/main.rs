fn main(){
    println!("Hello, world!");  
    let i = flatten(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]].into_iter());
    // for i in i {
    //     println!("{}", i);
    // }
}


pub trait IteratorExt: Iterator + Sized {
    fn our_flatten(self) -> Flatten<Self>
        where Self::Item: IntoIterator
    {
       flatten(self)
    }
}

impl<T> IteratorExt for T where T: Iterator {
    fn our_flatten(self) -> Flatten<Self>
        where Self::Item: IntoIterator
    {
       flatten(self)
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where 
    I: IntoIterator,
    I::Item: IntoIterator
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O> 
where
    O: Iterator,
    O::Item: IntoIterator
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O> 
where
    O: Iterator,
    O::Item: IntoIterator
{
    pub fn new(iter: O) -> Flatten<O> {
        Flatten { outer: iter, front_iter: None, back_iter: None }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.front_iter.as_mut() {
                if let Some(item) = inner_iter.next() {
                    return Some(item);
                }
                self.front_iter = None
            }

            if let Some(next_inner_iter) = self.outer.next() {
                self.front_iter = Some(next_inner_iter.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.back_iter.as_mut() {
                if let Some(item) = inner_iter.next_back() {
                    return Some(item);
                }
                self.back_iter = None
            }

            if let Some(next_inner_iter) = self.outer.next_back() {
                self.back_iter = Some(next_inner_iter.into_iter());
            } else {
                return self.front_iter.as_mut()?.next();
            }
        }
    }
    
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }
    
    #[test]
    fn empty_wide(){
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn one () {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two () {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide () {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }


    #[test]
    fn reverse () {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).rev().collect::<Vec<_>>(), vec!["b", "a"]);
    }

    #[test]
    fn reverse_wide () {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).rev().collect::<Vec<_>>(), vec!["b", "a"]);
    }

    #[test]
    fn both_ends(){
        let mut iter = vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]].into_iter().our_flatten();
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn inf(){
        let mut iter = flatten((0..).map(|i| 0..i));
        // 0 => 0..0 [0]
        // 1 => 0..1 [0, 1]
        // 2 => 0..2 [0, 1, 2]
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }


    #[test]
    fn deep(){
        assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
    }


}