use crate::GetQueryRole;
use std::fs;

struct TextData {
    title: String,
    text: String,
    image: String,
}

fn read_file(role: &GetQueryRole) -> Option<TextData> {
    let dir: String;
    match role {
        GetQueryRole::Family => dir = String::from("family"),
        GetQueryRole::Friend => dir = String::from("friend"),
        GetQueryRole::Random => dir = String::from("random"),
    }

    let title = fs::read_to_string(format!("./data/{}/title.txt", dir)).unwrap();
    let text = fs::read_to_string(format!("./data/{}/text.txt", dir)).unwrap();
    let image = fs::read_to_string(format!("./data/{}/image.txt", dir)).unwrap();
    if title.len() == 0 && text.len() == 0 && image.len() == 0 {
        return None;
    }

    Some(TextData {
        title: title,
        text: text,
        image: image,
    })
}

pub fn generate_html(role: GetQueryRole) -> String {
    let data: TextData;
    let default: String = format!(
        r#"<!DOCTYPE html>
        <body style="text-align: center; font-family:'Arial'">
            <h1 class="main-title">
                What random6 is doing now
            </h1>
            <h3>
                I Don't know :(
            </h3>
        </body>"#
    );

    match read_file(&role) {
        Some(text_data) => data = text_data,
        None => return default,
    };

    let role_text: String;
    match role {
        GetQueryRole::Family => role_text = String::from("Family"),
        GetQueryRole::Friend => role_text = String::from("Friend"),
        GetQueryRole::Random => role_text = String::from("Random"),
    }

    return format!(
        r#"
    <!DOCTYPE html>
<body style="text-align: center; font-family:'Arial'">
    <h1 class="main-title">
    {} - What random6 is doing now
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
        role_text, data.title, data.text, data.image
    );
}
