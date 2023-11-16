use std::ops;

/// A special data type that fixes the parent data type
/// in an immutable state in order to access the mutable
/// content.
pub struct Guard<'parent, 'child, Parent, Child>
where
    'parent: 'child,
{
    pub(crate) parent: &'parent Parent,
    pub(crate) child: &'child mut Child,
}

impl<'parent, 'child, Parent, Child> ops::Deref for Guard<'parent, 'child, Parent, Child>
where
    'parent: 'child,
{
    type Target = Child;

    fn deref(&self) -> &Self::Target {
        self.child
    }
}

impl<'parent, 'child, Parent, Child> ops::DerefMut for Guard<'parent, 'child, Parent, Child>
where
    'parent: 'child,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.child
    }
}

impl<'parent, 'child, Parent, Child, Idx> ops::Index<Idx> for Guard<'parent, 'child, Parent, Child>
where
    'parent: 'child,
    Child: ops::Index<Idx>,
{
    type Output = <Child as ops::Index<Idx>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        self.child.index(index)
    }
}

impl<'parent, 'child, Parent, Child, Idx> ops::IndexMut<Idx>
    for Guard<'parent, 'child, Parent, Child>
where
    'parent: 'child,
    Child: ops::IndexMut<Idx>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.child.index_mut(index)
    }
}

impl<'parent, 'child, Parent, Child> Guard<'parent, 'child, Parent, Child>
where
    'parent: 'child,
{
    pub fn parent(&self) -> &Parent {
        self.parent
    }
}
