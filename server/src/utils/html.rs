use std::fs;

struct TextData {
    title: String,
    text: String,
    image: String,
}

fn read_file() -> TextData {
    TextData {
        title: fs::read_to_string("./data/title.txt").unwrap(),
        text: fs::read_to_string("./data/text.txt").unwrap(),
        image: fs::read_to_string("./data/image.txt").unwrap(),
    }
}

pub fn generate_html() -> String {
    let text_data = read_file();

    format!(
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
        <img src="data:image/png;base64, {}" alt="image"/>
    </div>
</body>
    "#,
        text_data.title, text_data.text, text_data.image
    )
}
