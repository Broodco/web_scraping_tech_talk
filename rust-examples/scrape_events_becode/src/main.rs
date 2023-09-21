use scraper::ElementRef;

fn main() {
    // Step 1 : Get the url of the page you want to scrape
    let response = reqwest::blocking::get("https://becode.org/events");
    let content = response.unwrap().text().unwrap();

    // Step 2 : A little print to the console to test the result of the request
    //println!("{content}");

    // Step 3 : Parse the content into an HTML tree object
    let document = scraper::Html::parse_document(&content);

    // Step 4 : Establish a strategy to find the correct HTML elements you need
    let html_events_selector = scraper::Selector::parse("div.eaw-li__wrap").unwrap();
    let html_events = document.select(&html_events_selector);

    // Step 5 : Create a vector containing the data of each event
    let mut becode_events: Vec<BecodeEventStruct> = Vec::new();

    // Step 6 : Iterate over the list of events and store the relevant data into a struct
    for html_event in html_events {
        // Step 7 : Add the struct containing event information into the events vector
        becode_events.push(get_next_event_struct(html_event));
    }
    // Step 8 : Create the CSV output file
    let path = std::path::Path::new("becode_events.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    // Step 9 : Append the header to the CSV
    writer
        .write_record(&["url", "title", "image", "datetime"])
        .unwrap();

    // Step 10 : Write the data retrieved to the CSV
    for event in becode_events {
        let url = event.url.unwrap();
        let title = event.title.unwrap();
        let image = event.image.unwrap();
        let datetime = event.datetime.unwrap();

        writer.write_record(&[url, title, image, datetime]).unwrap();
    }

    // Step 11 : Free resources by flushing the writer
    writer.flush().unwrap();
}

fn get_next_event_struct(html_event: ElementRef) -> BecodeEventStruct {
    let url = html_event
        .select(&scraper::Selector::parse("a.eaw-img").unwrap())
        .next()
        .and_then(|a| a.value().attr("href"))
        .map(str::to_owned);
    let title = html_event
        .select(&scraper::Selector::parse("h3>a").unwrap())
        .next()
        .map(|title| title.inner_html());
    let image = html_event
        .select(&scraper::Selector::parse("a.eaw-img>img").unwrap())
        .next()
        .and_then(|a| a.value().attr("src"))
        .map(str::to_owned);
    let datetime = html_event
        .select(&scraper::Selector::parse("time").unwrap())
        .next()
        .map(|datetime| datetime.inner_html());

    let becode_event = BecodeEventStruct {
        url, title, image, datetime
    };
    becode_event
}


struct BecodeEventStruct {
    url: Option<String>,
    title: Option<String>,
    image: Option<String>,
    datetime: Option<String>
}