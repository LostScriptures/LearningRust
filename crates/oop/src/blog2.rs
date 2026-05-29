pub struct Post2 {
    content: String,
}

pub struct DraftPost2 {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

pub struct PreApprovedPost {
    content: String,
}

impl Post2 {
    pub fn new() -> DraftPost2 {
        DraftPost2 {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost2 {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> PreApprovedPost {
        PreApprovedPost {
            content: self.content,
        }
    }
}

impl PreApprovedPost {
    pub fn approve(self) -> Post2 {
        Post2 {
            content: self.content,
        }
    }
}
