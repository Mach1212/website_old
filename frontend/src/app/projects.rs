use zoon::named_color::TRANSPARENT;
use zoon::*;

use crate::app::{projects, svg_link, ACCENT_BACK, CORNER_RADIUS, SIZE, SPACING, TEXT};

pub fn page() -> impl Element {
    Column::new()
        .s(Font::new().color(TEXT[0]).size(SIZE[6]))
        .item(projects())
}

fn projects() -> impl Element {
    super::section(TRANSPARENT, SPACING[4], Column::new()
        .item(
            super::h2("Public Projects"),
        )
        .item(Column::new().s(Gap::new().y(SPACING[4])).s(Padding::new().top(SPACING[5]))
            .item(project("Website", "The website you are on now. I designed it in Figma and created it in MoonZoon, a Rust fullstack web framework. It is currently deployed to Netlify, but will be on AWS via the CDK", Some("https://github.com/Mach1212/website.git"), None, "mzoon.svg"))
            .item(project("Snake", "A snake game written in Bevy a Rust game engine", Some("snake_url"), Some("https://mpruchn.com/snake"), ""))
        ))
}

fn project(
    title: &str,
    description: &str,
    github: Option<&str>,
    url: Option<&str>,
    image: &str,
) -> impl Element {
    Row::new()
        .s(Gap::both(SPACING[5]))
        .s(Padding::new().x(SPACING[7]).y(SPACING[6]))
        .s(Background::new().color(ACCENT_BACK))
        .s(RoundedCorners::all(CORNER_RADIUS))
        .item(
            Column::new()
                .s(Width::percent(50))
                .s(Gap::both(SPACING[3]))
                .s(Align::new().top().left())
                .item(Row::new().s(Gap::both(SPACING[4])).items({
                    let mut vec = vec![];

                    if let Some(link) = github {
                        vec.push(svg_link("icons/github.svg", link, "Github link", SIZE[10]))
                    }

                    if let Some(link) = url {
                        vec.push(svg_link("icons/demo.svg", link, "Demo link", SIZE[10]))
                    }

                    vec
                }))
                .item(
                    El::new()
                        .s(Font::new().weight(FontWeight::Bold))
                        .child(title),
                )
                .item(El::new().child(description)),
        )
        .item(
            Image::new()
                .s(Width::fill())
                .s(Align::new().right())
                .s(RoundedCorners::all(CORNER_RADIUS))
                .url(public_url(image))
                .description("An intriguing image"),
        )
}
