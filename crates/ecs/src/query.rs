use paste::paste;

pub struct TupleAccessIterator<A: Iterator, B: Iterator>(A, B);

impl<
    A: Iterator, B: Iterator
> Iterator for TupleAccessIterator<A, B> {
    type Item = (<A as Iterator>::Item, <B as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        guard!(let Some(a) = self.0.next() else { return None; });
        guard!(let Some(b) = self.1.next() else { return None; });
        Some((a, b))
    }
}

pub trait Searchable {
    type Iter: Iterator;

    fn query(self) -> Self::Iter;
}

impl<A: Iterator, B: Iterator> Searchable for (A, B) {
    type Iter = TupleAccessIterator<A, B>;

    fn query(self) -> Self::Iter {
        TupleAccessIterator(self.0, self.1)
    }
}

macro_rules! generate_query {
    ($([$type: ident, $id: tt]), +) => {

paste! {
    pub struct [<TupleAccessIterator $($type)+>]<
        $($type: Iterator),+
    >($($type),+);
}

paste! {
    impl<
        $($type: Iterator),+
    > Iterator for [<TupleAccessIterator $($type)+>]<$($type),+> {
        type Item = ($(<$type as Iterator>::Item),+);

        fn next(&mut self) -> Option<Self::Item> {
            $(
                paste! {
                    guard!(let Some([<$type _p>]) = self.$id.next() else { return None; });
                }
            )+
            
            Some((
                $(paste! { [<$type _p>] }),+
            ))
        }
    }
}

paste! {
    impl<
        $($type: Iterator),+
    > Searchable for ($($type),+) {
        type Iter = [<TupleAccessIterator $($type)+>]<$($type),+>;

        fn query(self) -> Self::Iter {
            [<TupleAccessIterator $($type)+>]($(self.$id),+)
        }
    }
}

    };
}

generate_query!([A, 0], [B, 1], [C, 2]);
generate_query!([A, 0], [B, 1], [C, 2], [D, 3]);
generate_query!([A, 0], [B, 1], [C, 2], [D, 3], [E, 4]);
generate_query!([A, 0], [B, 1], [C, 2], [D, 3], [E, 4], [F, 5]);
generate_query!([A, 0], [B, 1], [C, 2], [D, 3], [E, 4], [F, 5], [G, 6]);
generate_query!([A, 0], [B, 1], [C, 2], [D, 3], [E, 4], [F, 5], [G, 6], [H, 7]);
generate_query!([A, 0], [B, 1], [C, 2], [D, 3], [E, 4], [F, 5], [G, 6], [H, 7], [I, 8]);