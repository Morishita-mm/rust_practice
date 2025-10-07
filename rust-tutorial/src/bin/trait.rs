// trait : 共通の振る舞いを抽象的に定義する。Scalaにおけるtraitとほぼ同じだと思う
// javaで言うインタフェースのさらに高機能版って感じだと思う

use std::fmt::{Debug, Display};

fn main() {
    println!("Trait sample in rust");

    let tweet = Tweet {
        username: String::from("mizuki"),
        content: String::from("of couse, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // "（{}さんの文章をもっと読む）"
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 戻り値の型にもimpl制約をつけられる
// しかし、複数の型を返すことはできない（Summaryを実装しているとしても）
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Summaryトレイトを実装しているものを受けとるようにしている
// 引数の数が増えたとしても、それぞれの引数が独立してSummaryを実装しているか解決される
// &(imple Summary + Display)などとして複数のトレイトを実装していることを確認することもできる
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// トレイト境界構文
// Summaryを実装している任意の型Tを引数にとる
// 引数が増えた時に、それぞれの型についても解決されるので、具体的な型が同一である場合はこちらの書き方を使用するのが良い
// <T: Summary + Display>として、複数のトレイトを実装していることを確認することができる
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 複数のジェネリクス型の引数を持つ関数は、可読性が低くなる
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    100
}

// where句を使用して書き換え
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    100
}
