type ImplIteratorMut<'a, Item> =
    ::std::iter::Chain<::std::slice::IterMut<'a, Item>, ::std::slice::IterMut<'a, Item>>;

/// A simple trait to allow accessing two parts of a vector mutably at the same time
///
/// # Examples
/// ```
/// for i in 0..bodies.len() {
///     let (body, other_bodys) = bodys.split_one_mut(i);
///
///     for body2 in other_bodys {
///         body.apply_gravity(&body2);
///     }
/// }
/// ```
pub trait SplitOneMut {
    type Item;

    fn split_one_mut(
        self: &'_ mut Self,
        i: usize,
    ) -> (&'_ mut Self::Item, ImplIteratorMut<'_, Self::Item>);
}

impl<T> SplitOneMut for [T] {
    type Item = T;

    fn split_one_mut(
        self: &'_ mut Self,
        i: usize,
    ) -> (&'_ mut Self::Item, ImplIteratorMut<'_, Self::Item>) {
        let (prev, current_and_end) = self.split_at_mut(i);
        let (current, end) = current_and_end.split_at_mut(1);
        (&mut current[0], prev.iter_mut().chain(end))
    }
}


