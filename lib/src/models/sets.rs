use std::marker::PhantomData;

pub struct StreamingSet<T, F: FnMut() -> Option<T>> {
    f: F,
    phantom: PhantomData<T>,
}

impl<T, F: FnMut() -> Option<T>> StreamingSet<T, F> {
    pub fn new(f: F) -> Self {
        Self { f, phantom: PhantomData }
    }
}

impl<T, F: FnMut() -> Option<T>> Iterator for StreamingSet<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.f)()
    }
}

// pub struct StreamingUnionSet<T: PartialEq, S1: Iterator<Item=T>, S2: Iterator<Item=T>> {
//     s1: S1,
//     s2: S2
// }

// impl<T: PartialEq, S1: Iterator<Item=T>, S2: Iterator<Item=T>> Iterator for StreamingUnionSet<T, S1, S2> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             let s1 = self.s1.next();
//             let s2 = self.s2.next();

//             if s1.is_none() || s2.is_none() {
//                 return None;
//             } else if s1 == s2 {
//                 return s1;
//             }
//         }
//     }
// }
