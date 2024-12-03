use std::cell::RefCell;

mod blog;

thread_local! {
    static BLOGS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}


#[ic_cdk::update]
fn add_blog(new_blog: String) {
    BLOGS.with(|blogs: &RefCell<Vec<String>>| blogs.borrow_mut().push(new_blog));
}

#[ic_cdk::query]
fn get_blogs() -> Vec<String> {
    BLOGS.with(|blogs: &RefCell<Vec<String>>| blogs.borrow().clone())
}
