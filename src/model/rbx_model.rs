use crate::model::{InstanceKind, ModelError};
use crate::tree::{NodeRef, Tree};

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

enum PathSegment<'a> {
    Index(usize),
    Name(&'a str),
}

fn split_path(path: &str) -> Result<Vec<PathSegment>, ModelError> {
    if path.is_empty() {
        return Err(ModelError::InvalidPath);
    }

    path.split('.')
        .map(|segment| match segment.parse::<usize>() {
            Ok(index) => Ok(PathSegment::Index(index)),
            Err(_) => {
                if segment.chars().all(char::is_alphanumeric) {
                    Ok(PathSegment::Name(segment))
                } else {
                    Err(ModelError::InvalidPath)
                }
            }
        })
        .collect()
}

/// A full Roblox model
#[derive(Debug, Clone)]
pub struct RbxModel {
    pub(crate) meta: BTreeMap<String, String>,
    pub(crate) nodes: Tree<InstanceKind>,
}

impl RbxModel {
    /// Create a new empty model
    pub fn new() -> RbxModel {
        RbxModel {
            meta: BTreeMap::new(),
            nodes: Tree::new(),
        }
    }

    /// Get an instance from a model, based on a path to that instance.
    ///
    /// # Path Syntax
    ///
    /// - Components are separated by `/`
    /// - A component can be either an index or a name
    /// - An index component is a usize representing the Nth child
    /// - A name component is any non-number string matching a possible instance name
    pub fn get_path(&self, path: &str) -> Result<NodeRef<'_, '_, InstanceKind>, ModelError> {
        let parts = split_path(path)?;

        let mut nodes = self.nodes.roots();

        let mut cur = match &parts[0] {
            PathSegment::Index(index) => {
                nodes.nth(*index).ok_or(ModelError::NotFound)?
            }
            PathSegment::Name(name) => {
                let mut results = self
                    .nodes
                    .roots()
                    .filter(|inst| inst.name() == *name)
                    .collect::<Vec<_>>();
                if results.len() > 1 {
                    return Err(ModelError::AmbiguousPath);
                } else if results.len() == 0 {
                    return Err(ModelError::NotFound);
                } else {
                    results.remove(0)
                }
            }
        };

        for segment in &parts[1..] {
            let new_cur = {
                let children = cur.children();
                match segment {
                    PathSegment::Index(index) => {
                        children.into_iter().nth(*index).ok_or(ModelError::NotFound)?
                    }
                    PathSegment::Name(name) => {
                        let mut results = children
                            .into_iter()
                            .filter(|inst| inst.name() == *name)
                            .collect::<Vec<_>>();
                        if results.len() > 1 {
                            return Err(ModelError::AmbiguousPath);
                        } else if results.len() == 0 {
                            return Err(ModelError::NotFound);
                        } else {
                            results.remove(0)
                        }
                    }
                }
            };

            cur = new_cur;
        }

        Ok(cur)
    }

    /// Get a reference to the set of meta values for this model
    pub fn meta(&self) -> &BTreeMap<String, String> {
        &self.meta
    }

    /// Get a mutable reference to the set of meta values for this model
    pub fn meta_mut(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.meta
    }

    /// Get the instance tree of this model
    pub fn tree(&self) -> &Tree<InstanceKind> {
        &self.nodes
    }

    // /// Add a root instance to this model
    // pub fn add_root(&mut self, inst: OwnedInstance) {
    //     self.roots.push(inst)
    // }
    //
    // /// Remove a root instance from this model
    // pub fn remove_root(&mut self, inst: &OwnedInstance) {
    //     self.roots.retain(|item| !Rc::ptr_eq(item, inst));
    // }
}

impl Default for RbxModel {
    fn default() -> RbxModel {
        let mut out = RbxModel {
            meta: BTreeMap::default(),
            nodes: Tree::new(),
        };
        out.meta
            .insert("ExplicitAutoJoints".to_string(), "true".to_string());
        out
    }
}
