#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FSTree {
    root: String,
    children: Vec<FSTree>,
}

impl FSTree {
    pub fn new(root: impl Into<String>) -> Self {
        FSTree {
            root: root.into(),
            children: vec![],
        }
    }
}

impl FSTree {
    pub fn get_root(&self) -> &str {
        &self.root
    }

    pub fn get_childern(&self) -> &[FSTree] {
        &self.children
    }

    pub fn add_child(&mut self, child: FSTree) {
        self.children.push(child);
    }

    pub fn with_child(mut self, child: FSTree) -> Self {
        self.children.push(child);
        self
    }

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn find(&self, name: &str) -> Option<&FSTree> {
        if self.root == name {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find(name) {
                return found.into();
            }
        }
        None
    }

    pub fn iter_preorder(&self) -> PreOrder<'_> {
        PreOrder { stack: vec![self] }
    }
}

#[derive(Debug, Clone)]
pub struct PreOrder<'a> {
    stack: Vec<&'a FSTree>,
}

impl<'a> Iterator for PreOrder<'a> {
    type Item = &'a FSTree;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        for child in node.children.iter().rev() {
            self.stack.push(child);
        }

        Some(node)
    }
}
