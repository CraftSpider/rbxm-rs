use crate::model::{Error, Instance, Result};

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

enum PathSegment<'a> {
    Index(usize),
    Name(&'a str),
}

fn split_path(path: &str) -> Result<Vec<PathSegment>> {
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

#[derive(Debug, Clone)]
pub struct RbxModel {
    pub meta: HashMap<String, String>,
    pub roots: Vec<Rc<RefCell<Instance>>>,
}

impl RbxModel {
    pub fn new() -> RbxModel {
        RbxModel {
            meta: HashMap::new(),
            roots: Vec::new(),
        }
    }

    pub fn get_path(&self, path: &str) -> Result<Rc<RefCell<Instance>>> {
        let parts = split_path(path)?;

        let mut cur = match &parts[0] {
            PathSegment::Index(index) => Rc::clone(self.roots.get(*index).ok_or(Error::NotFound)?),
            PathSegment::Name(name) => {
                let results = self
                    .roots
                    .iter()
                    .filter(|inst| inst.borrow().kind.name() == *name)
                    .collect::<Vec<_>>();
                if results.len() > 1 {
                    return Err(Error::AmbiguousPath);
                }
                Rc::clone(results.get(0).ok_or(Error::NotFound)?)
            }
        };

        for segment in &parts[1..] {
            let new_cur = {
                let children = Ref::map(cur.borrow(), |inst| &inst.children);
                match segment {
                    PathSegment::Index(index) => {
                        Rc::clone(children.get(*index).ok_or(Error::NotFound)?)
                    }
                    PathSegment::Name(name) => {
                        let results = children
                            .iter()
                            .filter(|inst| inst.borrow().kind.name() == *name)
                            .collect::<Vec<_>>();
                        if results.len() > 1 {
                            return Err(Error::AmbiguousPath);
                        }
                        Rc::clone(results.get(0).ok_or(Error::NotFound)?)
                    }
                }
            };

            cur = new_cur;
        }

        Ok(cur)
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
