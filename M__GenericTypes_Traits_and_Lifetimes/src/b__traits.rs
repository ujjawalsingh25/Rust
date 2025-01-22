
pub struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// _____________________________________________________________________________________________________________________
// fn news_aggregator_tweet(tweet: Tweet) {                                                                             |
//     println!("News: {}, Piblished: {}", tweet.content, tweet.username);      //  Here, both code same or of          |
// }                                                           // same functionality but work  for different struct     |
// fn news_aggregator_news(news: NewsArticle) {                                                                         |
//     println!("News: {}, Piblished: {}", news.content, news.author);      // NOT Having "SHARED BEHAVIOUR"            |
// }                                                                             // even using "Generics" as below      |
// fn news_aggregator_news<T>(source: T) {                            // Error since generic like 'int' or 'char'       |
//     println!("News: {}, Piblished: {}", T.content, T.author);      // don't have data like 'author' or 'content'     |
// }                                                                // so not possible to have generics like this       |
// _____________________________________________________________________________________________________________________|

//----------------- // For "SHARED BEHAVIOUR" -->> using traits  -------------       
pub trait Summary {
    fn get_author(&self) -> &str;
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {         // Default Trait
        format!("{}: (Read More)...", self.get_author())
    }
}

fn news_aggregator(source: &impl Summary) {                    // "Traits" as "Parameter"        
    println!("News below: \n{}", source.summarize());      
}                                          
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         let result = format!("Author: {}, Content: {}", self.username, self.content);
//         result
//     }
//     fn get_author(&self) -> &str {
//         &self.username.as_str()
//     }
// }
impl Summary for Tweet {                // If "impl" are not given as above then 
    fn get_author(&self) -> &str {          // takes the default "impl" i.e of "Read More"
        &self.username.as_str()
    }
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let result = format!("Author: {}, Content: {}", self.author, self.content);
        result
    }
    fn get_author(&self) -> &str {
        &self.author.as_str()
    }
}

pub fn traits_basic() {
    let tweet = Tweet {
        username: String::from("ujjawal@gamil.com"),
        content: String::from("Lerning Rust"),
        reply: false,
        retweet: false,
    };
    news_aggregator(&tweet);

    let news_article = NewsArticle {
        author: String::from("Ujjawal Singh"),
        content: String::from("From News_Article"),
        headline: String::from("Inside New_article"),
        location: String::from("Patna"),
    };
    news_aggregator(&news_article);

    mix_news_and_twitter(&tweet, &news_article);
} 



//---------  Traits Bound ("Shared Behaviour") with Generics  ---------    
fn get_news<T: Summary>(source: T) {
    println!("{}", source.summarize());
}

// fn mix_news_and_twitter<T: Summary>(primary_src: &T, other_src: &T) {
//     println!("{} and {}", primary_src.summarize(), other_src.summarize());
// }            
// "NO ERROR"  -->> Only when we try with same "Summary TYPE" say 
//                     ("NewsArticle" & "NewsArticle") or ("Tweet" & "Tweet" type)
// "ERROR"  -->> Only when we try with two different Summary ("NewsArticle" & "twitter")
//        Because of "TYPE MISMATCH" Tweet is a Tweet Type and NewsArticle is NewsArticle Type

fn mix_news_and_twitter(primary_src: &impl Summary, other_src: &impl Summary) {
    println!("{} and {}", primary_src.summarize(), other_src.summarize());
}
// ---------------------------------------------------------------------------------------------



//---------  Specifying Multiple Trait Bounds with the + Syntax  ---------  
pub trait Display {}  
// We can also specify more than one trait bound. Say we wanted notify to use display formatting 
// as well as summarize on item: we specify in the notify definition that item must implement
// both Display and Summary. We can do so using the + syntax:

pub fn notify(item: &(impl Summary + Display)) {}
// The + syntax is also valid with trait bounds on generic types:

pub fn notifyy<T: Summary + Display>(item: &T) {}
// With the two trait bounds specified, the body of notify can call summarize and use {} to format item.
// ---------------------------------------------------------------------------------------------


//---------  Clearer Trait Bounds with where Clauses  ---------    
pub trait Debug {} 
// Using too many trait bounds has its downsides. Each generic has its own trait bounds, 
// so functions with multiple generic type parameters can contain lots of trait bound 
// information between the function’s name and its parameter list, making the function 
// signature hard to read. For this reason, Rust has alternate syntax for specifying 
// trait bounds inside a where clause after the function signature. So, instead of writing this:

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {5}
// we can use a where clause, like this:
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    5
}
// ---------------------------------------------------------------------------------------------