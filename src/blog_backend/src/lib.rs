use std::cell::RefCell;

use crate::blog::Blog;

mod blog;

thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<Blog, String> {
    if title.len() > 250 {
        return Err("Title is too long.".to_string());
    } else if title.len() < 1 {
        return Err("Title is too short".to_string());
    }

    if content.len() > 500 {
        return Err("Content is too long".to_string());
    } else if content.len() < 1 {
        return Err("Content is too short".to_string());
    }

    if tags.len() > 3 {
        return Err("Max 3 tags allowed.".to_string());
    }

    BLOGS.with(|blogs: &RefCell<Vec<Blog>>| blogs.borrow_mut().push(Blog::new(title, content, tags)));

    let last_blog: Blog = BLOGS.with(|blogs: &RefCell<Vec<Blog>>|
        blogs
        .borrow()
        .last()
        .expect("Vector should not be empty!")
        .clone()
    );
    Ok(last_blog)
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs: &RefCell<Vec<Blog>>| blogs.borrow().clone())
}
