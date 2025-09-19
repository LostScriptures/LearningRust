pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(More from {})", self.summarize_author())
    }
}

pub struct Article {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.summarize_author())
    }
}

pub struct Post {
    pub username: String,
    pub content: String,
    pub repost: bool,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        if self.repost {
            format!("{} reposted {}", self.summarize_author(), self.content)
        } else {
            format!("{} posted {}", self.summarize_author(), self.content)
        }
    }
}

pub fn notify(item: &impl Summary) {
    // Short form of fn notify<T: Summary>(item: &T) {}
    // Can also be done with where: fn a<T, U>(ina: &T, inb: &U) where T: SomeTrait + SomeOtherTrait, U: ThirdTrait {}
    println!("{}", item.summarize());
}
