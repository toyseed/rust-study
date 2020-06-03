
struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        Box::new(PendingReview{})
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        post.content.as_str()
    }
}

#[test]
fn test_post() {
    let mut post = Post::new();
    post.add_content("hello");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("hello", post.content());
}