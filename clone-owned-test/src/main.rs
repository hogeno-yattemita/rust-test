use std::fmt;

fn main() {
    test_clone();
    test_to_owned();

    let m = "Hello.My.World";
    let s = first_sentence(m);
    println!("{}", s);
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl fmt::Display for ImportantExcerpt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.part)
    }
}

fn first_sentence(novel: &str) -> ImportantExcerpt<'_> {
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    ImportantExcerpt {
        part: first_sentence,
    }
}


fn test_clone() {
    let s1: &'static str = "I am static";
    let s2 = "I am boxed and owned".to_string();

    let c1 = s1.clone();
    let c2 = s2.clone();

    println!("{:?}", c1);
    println!("{:?}", c2);

    println!("{:?}", c1 == s1);  // prints true
    println!("{:?}", c2 == s2);  // prints true
}

fn test_to_owned() {
    let s1: &'static str = "I am static";
    let s2 = "I am boxed and owned".to_string();

    let c1 = s1.to_owned();
    let c2 = s2.to_owned();
    

    println!("{:?}", c1);
    println!("{:?}", c2);

    println!("{:?}", c1 == s1);   // compile-time error here (see below)
    println!("{:?}", c2 == s2);
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    // compiles and runs fine    
    fn test_clone() {
        let s1: &'static str = "I am static";
        let s2 = "I am boxed and owned".to_string();

        let c1 = s1.clone();
        let c2 = s2.clone();

        println!("{:?}", c1);
        println!("{:?}", c2);

        println!("{:?}", c1 == s1);  // prints true
        println!("{:?}", c2 == s2);  // prints true
    }

    #[test]
    fn test_to_owned() {
        let s1: &'static str = "I am static";
        let s2 = "I am boxed and owned".to_string();

        let c1 = s1.to_owned();
        let c2 = s2.to_owned();
    

        println!("{:?}", c1);
        println!("{:?}", c2);

        println!("{:?}", c1 == s1);   // compile-time error here (see below)
        println!("{:?}", c2 == s2);
    }
}