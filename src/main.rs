use scraper::{Html, Selector};

fn main() {
    println!("Hello, world!");
    let url = "https://tastesbetterfromscratch.com/pulled-pork/";
    let response = reqwest::blocking::get(url).expect("could not load url");
    let body = response.text().unwrap();
    //let selector = Selector::parse("script").unwrap();

    // i have not done scraping with scaper before
    let fragment = Html::parse_fragment(&body);
    let div_selector = Selector::parse(".wprm-recipe-ingredients-container").unwrap();
    let ingredients_container = fragment.select(&div_selector).next();

    let ingredients = ingredients_container.expect("REASON").text().collect::<Vec<_>>();
    println!("{:?}", ingredients);
    // ["Ingredients", "\u{a0}", "US Customary", "Metric", "\u{a0}", 
    // "▢ ","2", " ", "Tablespoons", " ", "oil (optional if searing)", "▢ ", "1", " ", "Tbsp", " ", "brown sugar", "▢ ", "1", " ", "tablespoon", " ", "chili powder", "▢ ", "1", " ", "teaspoon", " ", "onion powder", 
    // "▢ ", "1", " ", "teaspoon", " ", "garlic powder" "▢ ", "1", " ", "teaspoon", " ", "black pepper",
    // "▢ ", "1", " ", "teaspoon", " ", "kosher salt", 
    // that's how it comes in as a row 

    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();
    
    // lets just see if i can print out all list items
    let ul = fragment.select(&ul_selector).next().unwrap();
    for element in ul.select(&li_selector) {
        assert_eq!("li", element.value().name());
        let texty = element.text().collect::<Vec<_>>();
        println!("{:?}", &texty); // great
        //println!("{:?}", element.value().attr("text"));
    }
    
    
    // algorigthm could go
    // find the word ingredients / nav up a container / is there a list of text below?
    // first lets just use a css selector 
    //class="wprm-recipe-ingredients-container"
    // div[]
}
