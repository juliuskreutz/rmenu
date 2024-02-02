pub struct Window {
    pub width: u16,
    pub id: u32,
}

impl Window {
    pub fn new(width: u16, id: u32) -> Self {
        Self { width, id }
    }
}

pub struct Prompt {
    pub width: u16,
    pub text: String,
}

impl Prompt {
    pub fn new(width: u16, text: String) -> Self {
        Self { width, text }
    }
}

pub struct TextField {
    pub width: u16,
    pub text: String,
}

impl TextField {
    pub fn new() -> TextField {
        Self {
            width: crate::config::TEXT_FIELD_WIDTH,
            text: String::new(),
        }
    }
}

#[derive(Clone)]
pub struct Item {
    pub width: u16,
    pub text: String,
}

impl Item {
    pub fn new(width: u16, text: String) -> Self {
        Self { width, text }
    }
}

pub struct SearchField {
    pub width: u16,
    pub all_items: Vec<Item>,
    pub items: Vec<Item>,
    pub cursor: usize,
}

impl SearchField {
    pub fn new(width: u16, items: Vec<Item>) -> Self {
        Self {
            width,
            all_items: items,
            items: Vec::new(),
            cursor: 0,
        }
    }

    pub fn get_selection(&self) -> String {
        if let Some(item) = self.items.get(self.cursor) {
            item.text.clone()
        } else {
            "".to_string()
        }
    }
}
