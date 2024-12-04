use std::cell::RefCell;

use crate::blog::Blog;
use crate::config::Config;

mod config;
mod blog;

thread_local! {
    static CONFIG: RefCell<Config> = RefCell::new(Config::new());
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn add_config(_config: Config) {
    CONFIG.with(|config: &RefCell<Config>| *config.borrow_mut() = _config);
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<Blog, String> {
    let config: Config = CONFIG.with(|config: &RefCell<Config>| config.borrow().clone());

    if title.len() > config.max_title_length as usize {
        return Err("Title is too long.".to_string());
    } else if title.is_empty() {
        return Err("Title cannot be empty.".to_string());
    }

    if content.len() > config.max_content_length as usize {
        return Err("Content is too long.".to_string());
    } else if content.is_empty() {
        return Err("Content cannot be empty.".to_string());
    }

    if tags.len() > config.max_tags_count as usize {
        return Err("Max 3 tags allowed.".to_string());
    }

    let are_tags_in_config_tags: bool = tags.iter().any(|tag: &String| {
        !config.tags.contains(tag)
    });

    if are_tags_in_config_tags {
        return Err("Tags are not valid!".to_string());
    }


    BLOGS.with(|blogs: &RefCell<Vec<Blog>>| blogs.borrow_mut().push(Blog::new(title.clone(), content.clone(), tags.clone())));

    let last_blog: Blog = BLOGS.with(|blogs: &RefCell<Vec<Blog>>|
        blogs
        .borrow()
        .last()
        .expect("Vector should not be empty!")
        .clone()
    );

    ic_cdk::println!("Added new blog: (title: {}, content: {}, tags: {:?})", title, content, tags);

    Ok(last_blog)
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs: &RefCell<Vec<Blog>>| blogs.borrow().clone())
}
