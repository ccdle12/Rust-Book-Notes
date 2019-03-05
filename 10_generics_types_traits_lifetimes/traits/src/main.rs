// We cannot use external traits on external types.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// {}.. Will use the default summarize function.
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// summarize will override the default.
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        // format!("{}: {}", self.username, self.content)
        format!(
            "Read more of {} from {}",
            self.content,
            self.summarize_author()
        )
    }
}

/// Summary equivalent to an interface in go, c++, java.
pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;

    // Default implementations can call other methods in the same trait.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

/// Using traits as arguments in functions.
/// It is syntactic sugare for the below.
pub fn notify(item: impl Summary) {
    println!("Breaking News! {}...", item.summarize())
}

/// When we would be better to use the verbose form?
/// ...when we want to enforce the same type that implements Summary.
// pub fn notify<T: Summary>(item: T){...}

// Remember generics? since we specified T and T, both types MUST be the same.
// pub fn notify<T:  Summary>(item1: T, item2: T) {...}

/// Specifying multiple traits
// pub fn notify(item: impl Summary + Display){...}
// pub fn notify<T: Summary>(item: T){...}

/// Cleaner way to write multiple traits.
// fn some_function<T, U>(t: T, u: U) -> i32
// where
//     T: Summary + Clone,
//     U: Clone + Debug,
// {
//     i32::from(3)
// }

/// Returning Traits.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("tweet summary: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
        ),
    };

    // This will call the default.
    println!("New article available! {}", article.summarize());

    // Passing a trait.
    notify(article);

    let tweet = returns_summarizable();
    println!("tweet summarized: {}", tweet.summarize());
    notify(tweet);
}
