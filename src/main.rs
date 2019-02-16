extern crate reqwest;
extern crate select;

use reqwest::get;
use select::document::Document;
use select::predicate::{Class, Name};

#[derive(Debug)]
struct Case {
    title: String,
    text: Option<String>,
}

fn main() {
    let mut response = match get("http://thecodelesscode.com/case/random") {
        Ok(t) => t,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    if response.status() != 200 {
        println!(
            "Got response code {} for {}",
            response.status(),
            response.url()
        );
        std::process::exit(1);
    }

    let body = match response.text() {
        Ok(t) => t,
        Err(e) => {
            println!("Could not parse body, got error: {}", e);
            std::process::exit(1)
        }
    };

    let document = Document::from(&body[..]);

    if let Some(case) = get_case(&document) {
        println!("{}\n", case.title);
        println!("{}", case.text.unwrap());
    }
}

fn get_case(document: &Document) -> Option<Case> {
    get_case_with_title(&document).map(|mut case| {
        case.text = get_koan_text(&document);

        case
    })
}

fn get_case_with_title(document: &Document) -> Option<Case> {
    let title = document.find(Name("title"));

    title
        .last()
        .and_then(|node| node.first_child())
        .map(|first_child| Case {
            title: get_text(first_child),
            text: None,
        })
}

fn get_koan_text(document: &Document) -> Option<String> {
    let found = document.find(Class("koan"));

    let mut text = String::from("");

    found.for_each(|node| {
        text.push_str(&get_text(node)[..]);
    });

    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn get_text(node: select::node::Node) -> String {
    node.text().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_title_returns_case_when_title_exists() {
        let document = Document::from(
            "<html>\
             <head>\
             <title>\
             Foo\nBar Baz\
             </title>\
             </head>\
             <body>\
             </body>\
             </hmtl>",
        );

        let case_with_title = get_case_with_title(&document);

        assert_eq!(case_with_title.is_some(), true);
        assert_eq!(case_with_title.unwrap().title, "Foo\nBar Baz")
    }

    #[test]
    fn get_title_returns_none_when_no_title_exists() {
        let document = Document::from("<html><head></head><body></body></hmtl>");

        let case_with_title = get_case_with_title(&document);

        assert_eq!(case_with_title.is_none(), true);
    }

    #[test]
    fn get_koan_text_returns_koan_text_if_exists() {
        let document = Document::from(
            "<html>\
			<head>\
				<title>\
					Foo\nBar Baz\
				</title>\
			</head>\
			<body>\
				<div class=\"koan\">lorem ipsum\ndolor sit amet.</div>
			</body>\
		</hmtl>",
        );

        let koan = get_koan_text(&document);

        assert_eq!(koan.is_some(), true);
        assert_eq!(koan.unwrap(), "lorem ipsum\ndolor sit amet.");
    }

    #[test]
    fn get_koan_text_returns_none_if_no_koan_text() {
        let document = Document::from(
            "<html>\
             <head>\
             <title>\
             Foo\nBar Baz\
             </title>\
             </head>\
             <body>\
             </body>\
             </hmtl>",
        );

        let koan = get_koan_text(&document);

        assert_eq!(koan.is_none(), true);
    }

    #[test]
    fn get_case_return_case() {
        let document = Document::from(
            "<html>\
			<head>\
				<title>\
					Foo\nBar Baz\
				</title>\
			</head>\
			<body>\
				<div class=\"koan\">lorem ipsum\ndolor sit amet.</div>
			</body>\
		</hmtl>",
        );

        let case = get_case(&document);

        assert_eq!(case.is_some(), true);

        let case = case.unwrap();

        assert_eq!(case.title, "Foo\nBar Baz");
        assert_eq!(case.text.is_some(), true);

        let text = case.text.unwrap();

        assert_eq!(text, "lorem ipsum\ndolor sit amet.");
    }
}
