use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("mpruchn").append_to_head("\
<meta name=\"author\" content=\"Maciej Pruchnik\"/>
<meta name=\"description\" content=\"My personal website\"/>
<link rel=\"icon\" type=\"image/x-icon\" href=\"/_api/public/favicon.ico\"
<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">
<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>
<link href=\"https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,400;0,700;1,400&display=swap\" rel=\"stylesheet\">\
")
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
