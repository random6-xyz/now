use std::fs;

struct TextData {
    title: String,
    text: String,
    image: String,
}

fn read_file() -> Option<TextData> {
    let title = fs::read_to_string("./data/title.txt").unwrap();
    let text = fs::read_to_string("./data/text.txt").unwrap();
    let image = fs::read_to_string("./data/image.txt").unwrap();
    if title.len() == 0 && text.len() == 0 && image.len() == 0 {
        return None;
    }
    Some(TextData {
        title: title,
        text: text,
        image: image,
    })
}

pub fn generate_html() -> String {
    match read_file() {
        Some(text_data) => {
            return format!(
                r#"
            <!DOCTYPE html>
        <body style="text-align: center; font-family:'Arial'">
            <h1 class="main-title">
                What random6 is doing now
            </h1>
            <div class="title">
                <h3>
                    {}
                </h3>
            </div>
            <div class="text">
                <p>
                    {}
                </p>
            </div>
            <div class="image">
                <img src="data:image/png;base64, {}" alt="No image"/>
            </div>
        </body>
            "#,
                text_data.title, text_data.text, text_data.image
            );
        }
        None => {
            return format!(
                r#"<!DOCTYPE html>
        <body style="text-align: center; font-family:'Arial'">
            <h1 class="main-title">
                What random6 is doing now
            </h1>
            <h3>
                I Don't know :(
            </h3>
            "#
            );
        }
    }
}
