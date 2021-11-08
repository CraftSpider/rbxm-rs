
use core::cell::{RefCell, Ref, RefMut};
use core::ptr::NonNull;
use std::ops::{Deref, DerefMut};

use slotmap::{SlotMap, new_key_type};

macro_rules! ref_common {
    ($ty:ty) => {
        impl<'a, 'b, T> $ty {
            pub fn key(&self) -> TreeKey {
                self.mykey
            }

            pub fn parent(&self) -> Option<NodeRef<'a, 'b, T>> {
                self.node.parent
                    .and_then(|key| self.tree.try_get(key))
            }

            pub fn parent_mut(&self) -> Option<NodeRefMut<'a, 'b, T>> {
                self.node.parent
                    .and_then(|key| self.tree.try_get_mut(key))
            }

            pub fn children(&self) -> Vec<NodeRef<'a, 'b, T>> {
                self.node.children
                    .iter()
                    .map(|key| self.tree.try_get(*key).unwrap())
                    .collect::<Vec<_>>()
            }
        }
    }
}

new_key_type! {
    pub struct TreeKey;
}

#[derive(Clone, Debug)]
pub struct Tree<T> {
    inner: RefCell<InnerTree<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            inner: RefCell::new(InnerTree::new())
        }
    }

    fn inner_new_child(&self, item: T, parent: TreeKey) -> TreeKey {
        let mut rc = self.inner
            .borrow_mut();

        let new_node = RefCell::new(Node {
            parent: Some(parent),
            children: Vec::new(),
            item,
        });

        // SAFETY: Box::into_raw is guaranteed to return non-null pointer
        let new_node = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(new_node))) };

        rc.nodes.insert(new_node)
    }

    fn inner_set_child(&self, parent: &mut NodeRefMut<'_, '_, T>, child: &mut NodeRefMut<'_, '_, T>) {
        let mut rc = self.inner
            .borrow_mut();

        // Remove child's existing parent (remove it as a root, if it had no parent)
        match child.parent_mut() {
            None => rc.roots.retain(|key| *key != child.mykey),
            Some(mut parent) => parent.node.children.retain(|key| *key != child.mykey),
        }

        parent.node.children.push(child.mykey);
        child.node.parent = Some(parent.mykey);
    }

    fn inner_remove_child(&self, parent: &mut NodeRefMut<'_, '_, T>, child: &mut NodeRefMut<'_, '_, T>) {
        let mut rc = self.inner
            .borrow_mut();

        parent.node.children.retain(|key| *key != child.mykey);
        child.node.parent = None;

        rc.roots.push(child.mykey);
    }

    pub fn len(&self) -> usize {
        self.inner
            .borrow()
            .nodes
            .len()
    }

    pub fn try_get<'a, 'b>(&'a self, key: TreeKey) -> Option<NodeRef<'a, 'b, T>> {
        let inner = self.inner
            .borrow();

        let rc = inner
            .nodes
            .get(key)?;

        let r = unsafe { rc.as_ref() }.try_borrow().ok()?;

        Some(NodeRef {
            tree: self,
            mykey: key,
            node: r,
        })
    }

    pub fn try_get_mut<'a, 'b>(&'a self, key: TreeKey) -> Option<NodeRefMut<'a, 'b, T>> {
        let inner = self.inner
            .borrow();

        let rc = inner
            .nodes
            .get(key)?;

        // SAFETY: We only take immutable references to this data except when dropping
        //         Where we ensure no references live to any nodes
        let r = unsafe { rc.as_ref() }.try_borrow_mut().ok()?;

        Some(NodeRefMut {
            tree: self,
            mykey: key,
            node: r,
        })
    }

    pub fn unordered_iter(&self) -> impl Iterator<Item = NodeRef<'_, '_, T>> + '_ {
        let refs = self.inner
            .borrow()
            .nodes
            .iter()
            .map(|(key, item)| {
                NodeRef {
                    tree: self,
                    mykey: key,
                    node: unsafe { item.as_ref() }.borrow(),
                }
            })
            .collect::<Vec<_>>();

        refs.into_iter()
    }

    pub fn unordered_keys(&self) -> impl Iterator<Item = TreeKey> + '_ {
        let keys = self.inner
            .borrow()
            .nodes
            .keys()
            .collect::<Vec<_>>();

        keys.into_iter()
    }

    pub fn roots<'a, 'b>(&'a self) -> impl Iterator<Item = NodeRef<'a, 'b, T>>
    where
        T: 'b
    {
        let rc = self.inner
            .borrow();

        let refs = rc
            .roots
            .iter()
            .map(|key| {
                NodeRef {
                    tree: self,
                    mykey: *key,
                    node: unsafe { rc.nodes.get(*key).unwrap().as_ref() }.borrow(),
                }
            })
            .collect::<Vec<_>>();

        refs.into_iter()
    }

    pub fn roots_mut<'a, 'b>(&'a self) -> impl Iterator<Item = NodeRefMut<'a, 'b, T>>
    where
        T: 'b
    {
        let rc = self.inner
            .borrow();

        let refs = rc
            .roots
            .iter()
            .map(|key| {
                NodeRefMut {
                    tree: self,
                    mykey: *key,
                    node: unsafe { rc.nodes.get(*key).unwrap().as_ref() }.borrow_mut(),
                }
            })
            .collect::<Vec<_>>();

        refs.into_iter()
    }

    pub fn add_root(&self, item: T) -> TreeKey {
        let mut rc = self.inner
            .borrow_mut();

        let new_node = RefCell::new(Node {
            parent: None,
            children: Vec::new(),
            item,
        });

        let new_node = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(new_node))) };

        let new_key = rc.nodes.insert(new_node);
        rc.roots.push(new_key);
        new_key
    }
}

#[derive(Clone, Debug)]
pub struct InnerTree<T> {
    nodes: SlotMap<TreeKey, NonNull<RefCell<Node<T>>>>,
    roots: Vec<TreeKey>,
}

impl<T> InnerTree<T> {
    fn new() -> InnerTree<T> {
        InnerTree {
            nodes: SlotMap::with_key(),
            roots: Vec::new(),
        }
    }
}

pub struct Node<T> {
    parent: Option<TreeKey>,
    children: Vec<TreeKey>,
    item: T,
}

impl<T> Node<T> {

}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

pub struct NodeRef<'a, 'b, T: 'b> {
    tree: &'a Tree<T>,
    mykey: TreeKey,
    node: Ref<'b, Node<T>>,
}

ref_common! { NodeRef<'a, 'b, T> }

impl<T> Deref for NodeRef<'_, '_, T> {
    type Target = Node<T>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

pub struct NodeRefMut<'a, 'b, T: 'b> {
    tree: &'a Tree<T>,
    mykey: TreeKey,
    node: RefMut<'b, Node<T>>,
}

ref_common! { NodeRefMut<'a, 'b, T> }

impl<'a, 'b, T> NodeRefMut<'a, 'b, T> {
    pub fn set_parent(&mut self, parent: &mut NodeRefMut<'_, '_, T>) {
        self.tree.inner_set_child(parent, self);
    }

    pub fn new_child(&mut self, child: T) {
        let new_child = self.tree.inner_new_child(child, self.mykey);
        self.node.children.push(new_child);
    }

    pub fn add_child(&mut self, child: &mut NodeRefMut<'_, '_, T>) {
        self.tree.inner_set_child(self, child);
    }

    pub fn remove_child(&mut self, child: &mut NodeRefMut<'_, '_, T>) {
        self.tree.inner_remove_child(self, child);
    }
}

impl<T> Deref for NodeRefMut<'_, '_, T> {
    type Target = Node<T>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<T> DerefMut for NodeRefMut<'_, '_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tree() {
        let tree = Tree::<()>::new();
        assert!(tree.len() == 0);
        assert!(tree.roots().collect::<Vec<_>>().len() == 0);
    }

    #[test]
    fn tree_roots() {
        let tree = Tree::new();
        tree.add_root(true);
        tree.add_root(false);

        assert!(tree.len() == 2);
        assert!(tree.roots().collect::<Vec<_>>().len() == 2);
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

        assert!(tree.len() == 3);
        assert!(tree.roots().collect::<Vec<_>>().len() == 1);
    }
}
