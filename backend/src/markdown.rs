use pulldown_cmark::{Event, Parser, Tag};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct MarkdownAudit {
    pub word_count: u64,
    pub image_count: u64,
    pub heading_count: u64,
}

pub fn audit(markdown: &str) -> MarkdownAudit {
    let mut text = String::new();
    let mut image_count = 0_u64;
    let mut heading_count = 0_u64;

    for event in Parser::new(markdown) {
        match event {
            Event::Text(value) | Event::Code(value) => {
                text.push_str(&value);
                text.push(' ');
            }
            Event::Start(Tag::Image { .. }) => image_count += 1,
            Event::Start(Tag::Heading { .. }) => heading_count += 1,
            _ => {}
        }
    }

    let html_images = Regex::new(r"(?i)<img\b").expect("static image regex should compile");
    image_count += html_images.find_iter(markdown).count() as u64;

    MarkdownAudit {
        word_count: text.split_whitespace().count() as u64,
        image_count,
        heading_count,
    }
}

#[cfg(test)]
mod tests {
    use super::audit;

    #[test]
    fn audits_markdown_structure() {
        let result = audit("# Title\nHello **world**\n![alt](x.png)\n<img src=\"y.png\">");

        assert_eq!(result.word_count, 4);
        assert_eq!(result.image_count, 2);
        assert_eq!(result.heading_count, 1);
    }
}
