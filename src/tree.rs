use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::cell::{BorrowError, BorrowMutError};
use core::cell::{Ref, RefCell, RefMut};
use core::fmt;
#[cfg(feature = "unstable")]
use core::marker::Unsize;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use slotmap::{new_key_type, SecondaryMap, SlotMap};

type Result<T> = core::result::Result<T, Error>;

macro_rules! ref_common {
    ($ty:ty) => {
        impl<'a, 'b, T: ?Sized> $ty {
            pub fn key(&self) -> TreeKey {
                self.mykey
            }

            pub fn parent(&self) -> Result<Option<NodeRef<'a, 'b, T>>> {
                self.tree
                    .inner
                    .borrow()
                    .parents
                    .get(self.key())
                    .map(|&key| self.tree.try_get(key))
                    .transpose()
            }

            pub fn parent_mut(&self) -> Result<Option<NodeRefMut<'a, 'b, T>>> {
                self.tree
                    .inner
                    .borrow()
                    .parents
                    .get(self.key())
                    .map(|&key| self.tree.try_get_mut(key))
                    .transpose()
            }

            pub fn children(&self) -> impl Iterator<Item = Result<NodeRef<'a, 'b, T>>> {
                self.tree
                    .inner
                    .borrow()
                    .children
                    .get(self.key())
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|&key| self.tree.try_get(key))
                    .collect::<Vec<_>>()
                    .into_iter()
            }

            pub fn children_mut(&self) -> impl Iterator<Item = Result<NodeRefMut<'a, 'b, T>>> {
                self.tree
                    .inner
                    .borrow()
                    .children
                    .get(self.key())
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|&key| self.tree.try_get_mut(key))
                    .collect::<Vec<_>>()
                    .into_iter()
            }
        }

        impl<'a, 'b, T: ?Sized> Deref for $ty {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &*self.node
            }
        }

        impl<'a, 'b, T: ?Sized> AsRef<T> for $ty {
            fn as_ref(&self) -> &T {
                &*self.node
            }
        }

        impl<'a, 'b, T: ?Sized> Borrow<T> for $ty {
            fn borrow(&self) -> &T {
                &*self.node
            }
        }
    };
}

new_key_type! {
    pub struct TreeKey;
}

#[derive(Debug)]
pub enum Error {
    Missing,
    CantBorrow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Missing => write!(f, "Tree missing expected node"),
            Error::CantBorrow => write!(f, "Tree node is already borrowed incompatibly"),
        }
    }
}

impl From<BorrowError> for Error {
    fn from(_: BorrowError) -> Self {
        Error::CantBorrow
    }
}

impl From<BorrowMutError> for Error {
    fn from(_: BorrowMutError) -> Self {
        Error::CantBorrow
    }
}

#[derive(Clone)]
pub struct Tree<T: ?Sized> {
    inner: RefCell<InnerTree<T>>,
}

impl<T: ?Sized> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree::default()
    }

    pub fn len(&self) -> usize {
        self.inner.borrow().nodes.len()
    }

    #[cfg(feature = "unstable")]
    pub fn new_child_from<U: Unsize<T>>(&self, item: U, parent: TreeKey) {
        self.inner.borrow_mut().new_child_from(item, parent);
    }

    pub fn set_child(&self, parent: TreeKey, child: TreeKey) {
        self.inner.borrow_mut().set_child(parent, child);
    }

    pub fn remove_child(&self, parent: TreeKey, child: TreeKey) {
        self.inner.borrow_mut().remove_child(parent, child);
    }

    pub fn try_get<'a, 'b>(&'a self, key: TreeKey) -> Result<NodeRef<'a, 'b, T>> {
        let inner = self.inner.borrow();
        let rc = inner.nodes.get(key).ok_or(Error::Missing)?;
        NodeRef::try_borrow(self, key, rc)
    }

    pub fn try_get_mut<'a, 'b>(&'a self, key: TreeKey) -> Result<NodeRefMut<'a, 'b, T>> {
        let inner = self.inner.borrow();
        let rc = inner.nodes.get(key).ok_or(Error::Missing)?;
        NodeRefMut::try_borrow(self, key, rc)
    }

    pub fn unordered_iter(&self) -> impl Iterator<Item = Result<NodeRef<'_, '_, T>>> {
        self.inner
            .borrow()
            .nodes
            .iter()
            .map(|(key, item)| NodeRef::try_borrow(self, key, item))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn unordered_iter_mut(&self) -> impl Iterator<Item = Result<NodeRefMut<'_, '_, T>>> {
        self.inner
            .borrow()
            .nodes
            .iter()
            .map(|(key, item)| NodeRefMut::try_borrow(self, key, item))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn unordered_keys(&self) -> impl Iterator<Item = TreeKey> + '_ {
        self.inner
            .borrow()
            .nodes
            .keys()
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn roots<'a>(&'a self) -> impl Iterator<Item = Result<NodeRef<'a, '_, T>>> {
        let inner = self.inner.borrow();

        inner
            .roots
            .iter()
            .map(|key| {
                let node = inner.nodes.get(*key).ok_or(Error::Missing)?;
                NodeRef::try_borrow(self, *key, node)
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn roots_mut<'a>(&'a self) -> impl Iterator<Item = Result<NodeRefMut<'a, '_, T>>> {
        let inner = self.inner.borrow();

        inner
            .roots
            .iter()
            .map(|key| {
                let node = inner.nodes.get(*key).ok_or(Error::Missing)?;
                NodeRefMut::try_borrow(self, *key, node)
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn root_keys(&self) -> impl Iterator<Item = TreeKey> {
        self.inner
            .borrow()
            .roots
            .iter()
            .copied()
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<T> Tree<T> {
    fn inner_new_child(&self, item: T, parent: TreeKey) {
        self.inner.borrow_mut().new_child(item, parent);
    }

    pub fn add_root(&self, item: T) -> TreeKey {
        let mut rc = self.inner.borrow_mut();

        let new_node = RefCell::new(item);

        let new_node = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(new_node))) };

        let new_key = rc.nodes.insert(new_node);
        rc.roots.push(new_key);
        new_key
    }
}

fn recurse_tree<T: ?Sized + fmt::Debug>(
    f: &mut fmt::Formatter<'_>,
    indent: usize,
    node: Result<NodeRef<'_, '_, T>>,
) -> fmt::Result {
    match node {
        Ok(node) => {
            writeln!(f, "{}Node {{ {:?} }}", " ".repeat(indent), &*node)?;
            for child in node.children() {
                recurse_tree(f, indent + 4, child)?;
            }
        }
        Err(_) => writeln!(f, "{}Node {{ (Borrowed) }}", " ".repeat(indent))?,
    }
    Ok(())
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.roots() {
            recurse_tree(f, 0, node)?;
        }
        Ok(())
    }
}

impl<T: ?Sized> Default for Tree<T> {
    fn default() -> Self {
        Tree {
            inner: RefCell::new(InnerTree::new()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InnerTree<T: ?Sized> {
    nodes: SlotMap<TreeKey, NonNull<RefCell<T>>>,
    parents: SecondaryMap<TreeKey, TreeKey>,
    children: SecondaryMap<TreeKey, Vec<TreeKey>>,
    roots: Vec<TreeKey>,
}

impl<T: ?Sized> InnerTree<T> {
    fn new() -> InnerTree<T> {
        InnerTree {
            nodes: SlotMap::with_key(),
            parents: SecondaryMap::new(),
            children: SecondaryMap::new(),
            roots: Vec::new(),
        }
    }

    #[cfg(feature = "unstable")]
    fn new_child_from<U: Unsize<T>>(&mut self, item: U, parent: TreeKey) {
        let new_node = RefCell::from(item);

        // SAFETY: Box::into_raw is guaranteed to return non-null pointer
        let new_node =
            unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(new_node) as Box<RefCell<T>>)) };

        let new_key = self.nodes.insert(new_node);

        self.children
            .entry(parent)
            .unwrap()
            .or_default()
            .push(new_key);

        self.parents.insert(new_key, parent);
    }

    fn set_child(&mut self, parent: TreeKey, child: TreeKey) {
        let old_parent = self.parents.get(child);

        // Remove child's existing parent (remove it as a root, if it had no parent)
        match old_parent {
            Some(&old_parent) => self.children[old_parent].retain(|&k| k != child),
            None => self.roots.retain(|&k| k != child),
        }

        self.parents.insert(child, parent);
        self.children
            .entry(parent)
            .unwrap()
            .or_default()
            .push(child);
    }

    fn remove_child(&mut self, parent: TreeKey, child: TreeKey) {
        self.children[parent].retain(|&k| k != child);
        self.parents.remove(child);
        self.roots.push(child);
    }
}

impl<T> InnerTree<T> {
    fn new_child(&mut self, item: T, parent: TreeKey) {
        let new_node = RefCell::new(item);

        // SAFETY: Box::into_raw is guaranteed to return non-null pointer
        let new_node = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(new_node))) };

        let new_key = self.nodes.insert(new_node);

        self.children
            .entry(parent)
            .unwrap()
            .or_default()
            .push(new_key);

        self.parents.insert(new_key, parent);
    }
}

impl<T: ?Sized> Drop for InnerTree<T> {
    fn drop(&mut self) {
        for i in self.nodes.values() {
            unsafe {
                Box::from_raw(i.as_ptr());
            }
        }
    }
}

pub struct NodeRef<'a, 'b, T: ?Sized> {
    tree: &'a Tree<T>,
    mykey: TreeKey,
    node: Ref<'b, T>,
}

ref_common! { NodeRef<'a, 'b, T> }

impl<'a, 'b, T: ?Sized> NodeRef<'a, 'b, T> {
    fn try_borrow(
        tree: &'a Tree<T>,
        key: TreeKey,
        ptr: &'_ NonNull<RefCell<T>>,
    ) -> Result<NodeRef<'a, 'b, T>> {
        // SAFETY: We only take immutable references to this data except when dropping
        //         Where we ensure no references live to any nodes
        Ok(NodeRef {
            tree,
            mykey: key,
            node: unsafe { ptr.as_ref() }.try_borrow()?,
        })
    }
}

pub struct NodeRefMut<'a, 'b, T: ?Sized> {
    tree: &'a Tree<T>,
    mykey: TreeKey,
    node: RefMut<'b, T>,
}

ref_common! { NodeRefMut<'a, 'b, T> }

impl<'a, 'b, T: ?Sized> NodeRefMut<'a, 'b, T> {
    fn try_borrow(
        tree: &'a Tree<T>,
        key: TreeKey,
        ptr: &'_ NonNull<RefCell<T>>,
    ) -> Result<NodeRefMut<'a, 'b, T>> {
        // SAFETY: We only take immutable references to this data except when dropping
        //         Where we ensure no references live to any nodes
        Ok(NodeRefMut {
            tree,
            mykey: key,
            node: unsafe { ptr.as_ref() }.try_borrow_mut()?,
        })
    }

    #[cfg(feature = "unstable")]
    pub fn new_child_from<U: Unsize<T>>(&mut self, child: U) {
        self.tree.new_child_from(child, self.key());
    }

    pub fn set_parent(&mut self, parent: &NodeRef<'_, '_, T>) {
        self.tree.set_child(parent.key(), self.key());
    }

    pub fn add_child(&mut self, child: &NodeRef<'_, '_, T>) {
        self.tree.set_child(self.key(), child.key());
    }

    pub fn remove_child(&mut self, child: &NodeRef<'_, '_, T>) {
        self.tree.remove_child(self.key(), child.key());
    }
}

impl<T> NodeRefMut<'_, '_, T> {
    pub fn new_child(&mut self, child: T) {
        self.tree.inner_new_child(child, self.key());
    }
}

impl<T> DerefMut for NodeRefMut<'_, '_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.node
    }
}

impl<T> AsMut<T> for NodeRefMut<'_, '_, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut *self.node
    }
}

impl<T> BorrowMut<T> for NodeRefMut<'_, '_, T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut *self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tree() {
        let tree = Tree::<()>::new();
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.roots().collect::<Vec<_>>().len(), 0);
    }

    #[test]
    fn tree_roots() {
        let tree = Tree::new();
        tree.add_root(true);
        tree.add_root(false);

        assert_eq!(tree.len(), 2);
        assert_eq!(tree.roots().collect::<Vec<_>>().len(), 2);
    }

    #[test]
    fn tree_nodes() {
        let tree = Tree::new();
        tree.add_root(true);

        {
            let mut root = tree.roots_mut().next().unwrap().unwrap();
            root.new_child(true);
            root.new_child(false);
        }

        assert_eq!(tree.len(), 3);

        let roots = tree.roots().collect::<Result<Vec<_>>>().unwrap();

        assert_eq!(roots.len(), 1);
        assert_eq!(*roots[0], true);

        let children = roots[0].children().collect::<Result<Vec<_>>>().unwrap();

        assert_eq!(children.len(), 2);
    }
}
