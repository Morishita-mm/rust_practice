// トレイトオブジェクトを使用するとダイナミックディスパッチを使用するため、パフォーマンスが若干落ちる

// DrawトレイトとScreen構造体は、ステートパターンとは直接関係ありませんが、元のコードに残しておきます。
pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    // Vec内のどの要素も別の型であっても良いが、Drawトレイトを実装している必要がある
    pub components: Vec<Box<dyn Draw>>, // <----> impl Draw
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 以前の content メソッドは削除し、新しい content メソッドのみを残しました。
    // メソッドシグネチャはそのままにして、内部で Post への参照を渡すように修正します。
    pub fn content(&self) -> &str {
        // Post インスタンス自身（self）への不変な参照を渡すように修正
        // Post::content は &self を受け取っているので、
        // State::content に渡す際は &self とし、ライフタイムの問題を解決しています。
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // stateから所有権を一時的に取り出し、Stateメソッドを呼び出して新しいStateを受け取り、戻す
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// State トレイトの content メソッドのシグネチャを修正
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // 修正点: post を Post の所有権ではなく、ライフタイム 'a を持つ参照として受け取るように変更
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // 承認後は Published ステートへ移行
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // 修正点: Post の参照を受け取るようにシグネチャを修正し、Post の content フィールドを参照する
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    println!("OOP State Pattern in Rust");

    let mut post = Post::new();

    post.add_text("I ate a banana.");
    println!("Current content (Draft): {}", post.content()); // 空文字列が表示されるはず

    post.request_review();
    println!("Current content (PendingReview): {}", post.content()); // 空文字列が表示されるはず

    post.approve();
    println!("Current content (Published): {}", post.content()); // "I ate a banana." が表示されるはず
}
