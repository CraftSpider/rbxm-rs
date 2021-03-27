use super::OwnedInstance;
use crate::model::ModelError;

use std::cell::Ref;
use std::collections::HashMap;
use std::rc::Rc;

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
    pub(crate) meta: HashMap<String, String>,
    pub(crate) roots: Vec<OwnedInstance>,
}

impl RbxModel {
    /// Create a new empty model
    pub fn new() -> RbxModel {
        RbxModel {
            meta: HashMap::new(),
            roots: Vec::new(),
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
    pub fn get_path(&self, path: &str) -> Result<OwnedInstance, ModelError> {
        let parts = split_path(path)?;

        let mut cur = match &parts[0] {
            PathSegment::Index(index) => {
                Rc::clone(self.roots.get(*index).ok_or(ModelError::NotFound)?)
            }
            PathSegment::Name(name) => {
                let results = self
                    .roots
                    .iter()
                    .filter(|inst| inst.borrow().kind.name() == *name)
                    .collect::<Vec<_>>();
                if results.len() > 1 {
                    return Err(ModelError::AmbiguousPath);
                }
                Rc::clone(results.get(0).ok_or(ModelError::NotFound)?)
            }
        };

        for segment in &parts[1..] {
            let new_cur = {
                let children = Ref::map(cur.borrow(), |inst| &inst.children);
                match segment {
                    PathSegment::Index(index) => {
                        Rc::clone(children.get(*index).ok_or(ModelError::NotFound)?)
                    }
                    PathSegment::Name(name) => {
                        let results = children
                            .iter()
                            .filter(|inst| inst.borrow().kind.name() == *name)
                            .collect::<Vec<_>>();
                        if results.len() > 1 {
                            return Err(ModelError::AmbiguousPath);
                        }
                        Rc::clone(results.get(0).ok_or(ModelError::NotFound)?)
                    }
                }
            };

            cur = new_cur;
        }

        Ok(cur)
    }

    /// Get a reference to the set of meta values for this model
    pub fn meta(&self) -> &HashMap<String, String> {
        &self.meta
    }

    /// Get a mutable reference to the set of meta values for this model
    pub fn meta_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.meta
    }

    /// Get the root instances of this model
    pub fn roots(&self) -> &Vec<OwnedInstance> {
        &self.roots
    }

    /// Add a root instance to this model
    pub fn add_root(&mut self, inst: OwnedInstance) {
        self.roots.push(inst)
    }

    /// Remove a root instance from this model
    pub fn remove_root(&mut self, inst: &OwnedInstance) {
        self.roots.retain(|item| !Rc::ptr_eq(item, inst));
    }
}

impl Default for RbxModel {
    fn default() -> RbxModel {
        let mut out = RbxModel {
            meta: HashMap::default(),
            roots: Vec::default(),
        };
        out.meta
            .insert("ExplicitAutoJoints".to_string(), "true".to_string());
        out
    }
}
