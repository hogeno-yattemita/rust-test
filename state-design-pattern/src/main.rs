use state_design_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("{}", post.content());

    post.request_review();
    println!("{}", post.content());
    
    post.approve();
    println!("{}", post.content());
}
