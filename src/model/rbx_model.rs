use crate::model::{Instance, Error};
use crate::tree::{NodeRef, Tree};

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

enum PathSegment<'a> {
    Index(usize),
    Name(&'a str),
}

fn split_path(path: &str) -> Result<Vec<PathSegment<'_>>, Error> {
    if path.is_empty() {
        return Err(Error::InvalidPath);
    }

    path.split('.')
        .map(|segment| match segment.parse::<usize>() {
            Ok(index) => Ok(PathSegment::Index(index)),
            Err(_) => {
                if segment.chars().all(char::is_alphanumeric) {
                    Ok(PathSegment::Name(segment))
                } else {
                    Err(Error::InvalidPath)
                }
            }
        })
        .collect()
}

/// A full Roblox model
#[derive(Debug, Clone)]
pub struct RbxModel {
    pub(crate) meta: BTreeMap<String, String>,
    pub(crate) nodes: Tree<Instance>,
}

impl RbxModel {
    /// Create a new empty model
    #[must_use]
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
    pub fn get_path(&self, path: &str) -> Result<NodeRef<'_, '_, Instance>, Error> {
        let parts = split_path(path)?;

        let mut nodes = self
            .nodes
            .roots()
            .into_iter()
            .collect::<Result<Vec<_>, _>>();

        let mut out = None;

        for segment in parts {
            let new_next = match segment {
                PathSegment::Index(index) => {
                    nodes?.into_iter().nth(index).ok_or(Error::NotFound)?
                }
                PathSegment::Name(name) => {
                    let mut results = nodes?
                        .into_iter()
                        .filter(|inst| inst.name() == name)
                        .collect::<Vec<_>>();

                    if results.len() > 1 {
                        return Err(Error::AmbiguousPath);
                    } else if results.is_empty() {
                        return Err(Error::NotFound);
                    }

                    results.remove(0)
                }
            };

            nodes = new_next
                .children()
                .into_iter()
                .collect::<Result<Vec<_>, _>>();

            out = Some(new_next);
        }

        out.ok_or(Error::InvalidPath)
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
    pub fn tree(&self) -> &Tree<Instance> {
        &self.nodes
    }
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

/*#[cfg(test)]
mod tests {
    use crate::model::instance::{Base, Model};
    use super::*;

    #[test]
    fn test_get_path() {
        let model = RbxModel::new();
        model
            .tree()
            .add_root(Instance::Model(Model { base: Base::default(),  }));
    }
}*/
