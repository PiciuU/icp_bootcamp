use std::cell::RefCell;

use crate::blog::Blog;

mod blog;

thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) {
    let blog: Blog = Blog::new(title, content, tags);
    BLOGS.with(|blogs: &RefCell<Vec<Blog>>| blogs.borrow_mut().push(blog));
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs: &RefCell<Vec<Blog>>| blogs.borrow().clone())
}
