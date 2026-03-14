#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Item {
    pub name: String,
}

impl Item {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }
}
