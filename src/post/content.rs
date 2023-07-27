use reqwest::Url;
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

use crate::blog::BlogMention;

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Formatting {
    // Basic
    Bold {
        start: u32,
        end: u32,
    },
    Italic {
        start: u32,
        end: u32,
    },
    Strikethrough {
        start: u32,
        end: u32,
    },
    Small {
        start: u32,
        end: u32,
    },

    // Complex
    Link {
        start: u32,
        end: u32,
        url: Url,
    },
    Mention {
        start: u32,
        end: u32,
        blog: BlogMention,
    },
    Color {
        start: u32,
        end: u32,
        hex: String,
    },
}

/// https://www.tumblr.com/docs/npf#text-block-basic-subtypes
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContentSubtype {
    #[serde(rename = "heading1")]
    HeadingOne,
    #[serde(rename = "heading2")]
    HeadingTwo,
    Quirky,
    Quote,
    Indented,
    Chat,
    OrderedListItem,
    UnorderedListItem,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum PostContent {
    Text {
        text: String,
        subtype: Option<ContentSubtype>,
        indent_level: Option<u8>,
        formatting: Option<Vec<Formatting>>,
    },
    Image,
    Link,
    Audio,
    Video,
    Paywall,
}
