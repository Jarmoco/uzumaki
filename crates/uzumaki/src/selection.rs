use crate::element::UzNodeId;

#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SelectionRange {
    /// Anchor point (where selection started), flat grapheme index
    pub anchor: usize,
    /// Active point / cursor position, flat grapheme index
    pub active: usize,
}

impl SelectionRange {
    pub fn new(anchor: usize, active: usize) -> Self {
        Self { anchor, active }
    }

    pub fn is_collapsed(&self) -> bool {
        self.anchor == self.active
    }

    pub fn start(&self) -> usize {
        self.anchor.min(self.active)
    }

    pub fn end(&self) -> usize {
        self.anchor.max(self.active)
    }

    pub fn set_cursor(&mut self, pos: usize) {
        self.anchor = pos;
        self.active = pos;
    }
}

/// Text selection for non-input elements (textSelect views).
///
/// `root == None` means no active view selection. Input nodes own their own
/// selection internally and do not populate this.
#[derive(Debug, Copy, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TextSelection {
    pub root: Option<UzNodeId>,
    pub range: SelectionRange,
}

impl TextSelection {
    pub fn new(root: UzNodeId, anchor: usize, active: usize) -> Self {
        Self {
            root: Some(root),
            range: SelectionRange { anchor, active },
        }
    }

    pub fn is_active(&self) -> bool {
        self.root.is_some()
    }

    #[inline]
    pub fn anchor(&self) -> usize {
        self.range.anchor
    }

    #[inline]
    pub fn active(&self) -> usize {
        self.range.active
    }

    pub fn set_cursor(&mut self, pos: usize) {
        self.range.set_cursor(pos);
    }

    pub fn start(&self) -> usize {
        self.range.start()
    }

    pub fn end(&self) -> usize {
        self.range.end()
    }

    pub fn is_collapsed(&self) -> bool {
        self.range.is_collapsed()
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.range = SelectionRange::default();
    }
}
