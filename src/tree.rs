use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::{Ref, RefCell, RefMut};
use core::fmt;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use slotmap::{new_key_type, SecondaryMap, SlotMap};

macro_rules! ref_common {
    ($ty:ty) => {
        impl<'a, 'b, T> $ty {
            pub fn key(&self) -> TreeKey {
                self.mykey
            }

            pub fn parent(&self) -> Result<Option<NodeRef<'a, 'b, T>>, Error> {
                self.tree
                    .inner
                    .borrow()
                    .parents
                    .get(self.key())
                    .map(|&key| self.tree.try_get(key))
                    .transpose()
            }

            pub fn parent_mut(&self) -> Result<Option<NodeRefMut<'a, 'b, T>>, Error> {
                self.tree
                    .inner
                    .borrow()
                    .parents
                    .get(self.key())
                    .map(|&key| self.tree.try_get_mut(key))
                    .transpose()
            }

            pub fn children(&self) -> impl Iterator<Item = Result<NodeRef<'a, 'b, T>, Error>> {
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

            pub fn children_mut(
                &self,
            ) -> impl Iterator<Item = Result<NodeRefMut<'a, 'b, T>, Error>> {
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

#[derive(Clone)]
pub struct Tree<T> {
    inner: RefCell<InnerTree<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree::default()
    }

    fn inner_new_child(&self, item: T, parent: TreeKey) {
        let mut inner = self.inner.borrow_mut();

        let new_node = RefCell::new(item);

        // SAFETY: Box::into_raw is guaranteed to return non-null pointer
        let new_node = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(new_node))) };

        let new_key = inner.nodes.insert(new_node);

        inner
            .children
            .entry(parent)
            .unwrap()
            .or_default()
            .push(new_key);

        inner.parents.insert(new_key, parent);
    }

    fn inner_set_child(&self, parent: TreeKey, child: TreeKey) {
        let mut inner = self.inner.borrow_mut();

        let old_parent = inner.parents.get(child);

        // Remove child's existing parent (remove it as a root, if it had no parent)
        match old_parent {
            Some(&old_parent) => inner.children[old_parent].retain(|&k| k != child),
            None => inner.roots.retain(|&k| k != child),
        }

        inner.parents.insert(child, parent);
        inner.children
            .entry(parent)
            .unwrap()
            .or_default()
            .push(child);
    }

    fn inner_remove_child(&self, parent: TreeKey, child: TreeKey) {
        let mut inner = self.inner.borrow_mut();

        inner.children[parent].retain(|&k| k != child);

        inner.parents.remove(child);

        inner.roots.push(child);
    }

    pub fn len(&self) -> usize {
        self.inner.borrow().nodes.len()
    }

    pub fn try_get<'a, 'b>(&'a self, key: TreeKey) -> Result<NodeRef<'a, 'b, T>, Error> {
        let inner = self.inner.borrow();

        let rc = inner.nodes.get(key).ok_or(Error::Missing)?;

        let r = unsafe { rc.as_ref() }
            .try_borrow()
            .map_err(|_| Error::CantBorrow)?;

        Ok(NodeRef {
            tree: self,
            mykey: key,
            node: r,
        })
    }

    pub fn try_get_mut<'a, 'b>(&'a self, key: TreeKey) -> Result<NodeRefMut<'a, 'b, T>, Error> {
        let inner = self.inner.borrow();

        let rc = inner.nodes.get(key).ok_or(Error::Missing)?;

        // SAFETY: We only take immutable references to this data except when dropping
        //         Where we ensure no references live to any nodes
        let r = unsafe { rc.as_ref() }
            .try_borrow_mut()
            .map_err(|_| Error::CantBorrow)?;

        Ok(NodeRefMut {
            tree: self,
            mykey: key,
            node: r,
        })
    }

    pub fn unordered_iter(&self) -> impl Iterator<Item = NodeRef<'_, '_, T>> + '_ {
        self.inner
            .borrow()
            .nodes
            .iter()
            .map(|(key, item)| NodeRef {
                tree: self,
                mykey: key,
                node: unsafe { item.as_ref() }.borrow(),
            })
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

    pub fn roots<'a, 'b>(&'a self) -> impl Iterator<Item = Result<NodeRef<'a, 'b, T>, Error>>
    where
        T: 'b,
    {
        let rc = self.inner.borrow();

        rc.roots
            .iter()
            .map(|key| {
                let node = unsafe { rc.nodes.get(*key).ok_or(Error::Missing)?.as_ref() }
                    .try_borrow()
                    .map_err(|_| Error::CantBorrow)?;
                Ok(NodeRef {
                    tree: self,
                    mykey: *key,
                    node,
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn roots_mut<'a, 'b>(&'a self) -> impl Iterator<Item = NodeRefMut<'a, 'b, T>>
    where
        T: 'b,
    {
        let rc = self.inner.borrow();

        rc.roots
            .iter()
            .map(|key| NodeRefMut {
                tree: self,
                mykey: *key,
                node: unsafe { rc.nodes.get(*key).unwrap().as_ref() }.borrow_mut(),
            })
            .collect::<Vec<_>>()
            .into_iter()
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

fn recurse_tree<T: fmt::Debug>(
    f: &mut fmt::Formatter<'_>,
    indent: usize,
    node: Result<NodeRef<'_, '_, T>, Error>,
) -> fmt::Result {
    match node {
        Ok(node) => {
            writeln!(f, "{}Node {{ {:?} }}", " ".repeat(indent), *node)?;
            for child in node.children() {
                recurse_tree(f, indent + 4, child)?;
            }
        }
        Err(_) => writeln!(f, "{}Node {{ (Borrowed) }}", " ".repeat(indent))?,
    }
    Ok(())
}

impl<T: fmt::Debug> fmt::Debug for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.roots() {
            recurse_tree(f, 0, node)?;
        }
        Ok(())
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree {
            inner: RefCell::new(InnerTree::new()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InnerTree<T> {
    nodes: SlotMap<TreeKey, NonNull<RefCell<T>>>,
    parents: SecondaryMap<TreeKey, TreeKey>,
    children: SecondaryMap<TreeKey, Vec<TreeKey>>,
    roots: Vec<TreeKey>,
}

impl<T> InnerTree<T> {
    fn new() -> InnerTree<T> {
        InnerTree {
            nodes: SlotMap::with_key(),
            parents: SecondaryMap::new(),
            children: SecondaryMap::new(),
            roots: Vec::new(),
        }
    }
}

pub struct NodeRef<'a, 'b, T> {
    tree: &'a Tree<T>,
    mykey: TreeKey,
    node: Ref<'b, T>,
}

ref_common! { NodeRef<'a, 'b, T> }

impl<T> Deref for NodeRef<'_, '_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.node
    }
}

pub struct NodeRefMut<'a, 'b, T> {
    tree: &'a Tree<T>,
    mykey: TreeKey,
    node: RefMut<'b, T>,
}

ref_common! { NodeRefMut<'a, 'b, T> }

impl<'a, 'b, T> NodeRefMut<'a, 'b, T> {
    pub fn set_parent(&mut self, parent: &NodeRef<'_, '_, T>) {
        self.tree.inner_set_child(parent.key(), self.key());
    }

    pub fn new_child(&mut self, child: T) {
        self.tree.inner_new_child(child, self.key());
    }

    pub fn add_child(&mut self, child: &NodeRef<'_, '_, T>) {
        self.tree.inner_set_child(self.key(), child.key());
    }

    pub fn remove_child(&mut self, child: &NodeRef<'_, '_, T>) {
        self.tree.inner_remove_child(self.key(), child.key());
    }
}

impl<T> Deref for NodeRefMut<'_, '_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.node
    }
}

impl<T> DerefMut for NodeRefMut<'_, '_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
            let mut root = tree.roots_mut().next().unwrap();
            root.new_child(true);
            root.new_child(false);
        }

        assert_eq!(tree.len(), 3);

        let roots = tree.roots().collect::<Result<Vec<_>, _>>().unwrap();

        assert_eq!(roots.len(), 1);
        assert_eq!(*roots[0], true);

        let children = roots[0].children().collect::<Result<Vec<_>, _>>().unwrap();

        assert_eq!(children.len(), 2);
    }
}
