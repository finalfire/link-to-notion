pub mod entity {
    use serde::{Serialize};

    const TEXTUAL_TYPE: &str = "text";

    #[derive(Serialize)]
    struct TextualContent {
        content: String
    }

    #[derive(Serialize)]
    struct TextType {
        r#type: String,
        text: TextualContent
    }

    #[derive(Serialize)]
    struct Title {
        title: Vec<TextType>
    }

    impl Title {
        fn new(name: String) -> Self {
            let mut title = Vec::new();
            title.push(TextType { 
                r#type: TEXTUAL_TYPE.to_string(),
                text: TextualContent { content: name }
            });
            Title { title }
        }
    }

    #[derive(Serialize)]
    struct Url {
        url: String
    }

    #[derive(Serialize)]
    struct Tag {
        name: String
    }

    #[derive(Serialize)]
    struct Tags { 
        multi_select: Vec<Tag>
    }

    #[derive(Serialize)]
    struct DatabaseId {
        database_id: String
    }

    #[derive(Serialize)]
    struct Properties {
        #[serde(rename = "Title")]
        title: Title,
        #[serde(rename = "URL")]
        url: Url,
        #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
        tags: Option<Tags>,
    }

    #[derive(Serialize)]
    pub struct Item {
        parent: DatabaseId,
        properties: Properties
    }

    impl Item {
        pub fn new(database_id: String, title: String, url: String, tags_data: Option<Vec<String>>) -> Self {
            let database_id = DatabaseId { database_id };
            
            let title = Title::new(title);
            let url = Url { url };
            let tags = tags_data.map(|values| Tags {
                multi_select: values.iter().map(|t| Tag { name: t.to_string() }).collect()
            });

            Item { parent: database_id, properties: Properties { title, url, tags } }
        }
    }
}