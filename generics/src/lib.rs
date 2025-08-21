pub mod normal_trait {
    pub trait Summary {
        // 基础实现
        fn summarize(&self) -> String;
    }
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}

pub mod default_trait {
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    pub trait Summary {
        // 默认实现
        fn summarize(&self) -> String {
            // 默认没有定义的执行下面的语句
            String::from("read more")
        }
    }
    impl Summary for NewsArticle {}
}

// 内部调用其他的方法
pub mod call_trait {
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    pub trait Summary {
        // 调用其他的
        fn summarize(&self) -> String {
            format!("{}", self.summarize_author())
        }

        fn summarize_author(&self) -> String;
    }
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }
}
