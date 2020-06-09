use std::fmt::{Debug, Display};

trait Summary {
    fn summarize(&self) -> String;
    fn default_summarize(&self) -> String {
        format!("this is default: {}.", &self.summarize())
    }
}

#[derive(Debug)]
struct NewArticle {
    headlines: String,
    location: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{} {}", self.headlines, self.location)
    }
}

fn receive_trait(summary: &impl Summary) {
    println!("call summary() from receive_trait: {}", summary.summarize());
}

fn receive_trait_with_generic<T: Summary>(summary: &T) {
    println!(
        "call summary() from receive_trait_with_generic: {}",
        summary.summarize()
    );
}

fn some_function<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

fn returns_summarizable() -> impl Summary {
    NewArticle {
        headlines: String::from("hl"),
        location: String::from("lc"),
    }
}

trait SayHello {
    fn say_hello(&self);
}

// blanket
impl<T: Summary> SayHello for T {
    fn say_hello(&self) {
        println!("hello");
    }
}

#[test]
fn test_traits() {
    let article = NewArticle {
        headlines: String::from("headline"),
        location: String::from("location"),
    };
    println!("{}", article.summarize());
    println!("{}", article.default_summarize());

    receive_trait(&article);
    receive_trait_with_generic(&article);

    println!("{:?}", article);
    article.say_hello();
}

fn is_returning_trait_possible() -> impl Summary {
    NewArticle {
        headlines: "head".to_string(),
        location: "loc".to_string(),
    }
}

#[test]
fn test_call_above() {
    let summary = is_returning_trait_possible();
    println!("@@: {}", summary.summarize());
}