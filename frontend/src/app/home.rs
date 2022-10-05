use zoon::named_color::{GRAY_4, TRANSPARENT};
use zoon::row::{EmptyFlagNotSet, MultilineFlagNotSet};
use zoon::*;

use crate::app::{ACCENT, ACCENT_SHADE, ACCENT_TINT, CORNER_RADIUS, SIZE, SPACING, TEXT};
use crate::web_sys::HtmlElement;

pub fn page() -> impl Element {
    Column::new()
        .s(Font::new()
            .color(TEXT[0])
            .size(SIZE[6])
            .family([FontFamily::new("Open Sans"), FontFamily::SansSerif]))
        .item(introduction())
        .item(education())
        .item(experience())
        .item(languages())
        .item(resumes())
        .item(call_to_action())
}

fn introduction() -> impl Element {
    super::section(
        ACCENT_TINT,
        0,
        Row::new()
            .s(Padding::new().bottom(SPACING[5]))
            .s(Gap::new().x(SPACING[1]))
            .s(Align::center())
            .item(
                Column::new()
                    .s(Gap::both(SPACING[5]))
                    .item(
                        El::new()
                            .s(Width::fill().max(800))
                            .child("Hi, I am Maciej Pruchnik, a software engineer")
                            .s(Font::new().size(SIZE[12]).line_height(SPACING[9])),
                    )
                    .item(
                        El::new()
                            .child("Solving your problems with elegant solutions")
                            .s(Font::new().size(SIZE[5]).italic()),
                    )
                    .item(
                        Link::new()
                            .label(super::make_button(
                                "Email me",
                                ACCENT_TINT,
                                ACCENT,
                                ACCENT_SHADE,
                                || {},
                            ))
                            .to("mailto:mpruchn@ncsu.edu")
                            .new_tab(NewTab::new().follow(true))
                            .s(Align::new().left()),
                    ),
            )
            .item(Spacer::growable())
            .item(
                Image::new()
                    .url(public_url("self_website.jpg"))
                    .description("An image of myself")
                    .s(Width::fill().max(500))
                    .s(RoundedCorners::all(CORNER_RADIUS))
                    .s(Shadows::new([Shadow::new().color(GRAY_4).blur(SPACING[7])])),
            ),
    )
}

fn education() -> impl Element {
    super::section(
        TRANSPARENT,
        SPACING[4],
        Row::new().item(super::h2("Education")),
    )
}

fn experience() -> impl Element {
    super::section(
        TRANSPARENT,
        SPACING[4],
        Column::new().item(super::h2("Experiences")),
    )
}

fn languages() -> impl Element {
    super::section(
        TRANSPARENT,
        SPACING[4],
        Column::new()
            .s(Gap::both(SPACING[4]))
            .item(super::h2("Languages"))
            .item(
                Row::new()
                    .s(Background::new().color(ACCENT_TINT))
                    .s(RoundedCorners::all(CORNER_RADIUS))
                    .s(Padding::new().x(SPACING[7]).y(SPACING[6]))
                    .s(Align::new().center_x())
                    .s(Gap::both(SPACING[6]))
                    .item(super::svg("icons/rust.png", "Rust", SIZE[13]))
                    .item(super::svg("icons/java.png", "Java", SIZE[13]))
                    .item(super::svg("icons/javascript.svg", "Javascript", SIZE[13]))
                    .item(super::svg("icons/typescript.png", "Typescript", SIZE[13]))
                    .item(super::svg("icons/html.svg", "HTML", SIZE[13]))
                    .item(super::svg("icons/css.png", "CSS", SIZE[13])),
            ),
    )
}

fn resumes() -> impl Element {
    super::section(
        TRANSPARENT,
        SPACING[4],
        Column::new().item(super::h2("Resumes")).item(
            Row::new()
                .s(Background::new().color(ACCENT_TINT))
                .s(RoundedCorners::all(CORNER_RADIUS))
                .s(Padding::new()
                    .x(SPACING[7])
                    .top(SPACING[6])
                    .bottom(SPACING[5]))
                .s(Align::new().center_x())
                .s(Gap::both(SPACING[6]))
                .item(
                    Column::new()
                        .item(super::svg_link(
                            "icons/pdf.svg",
                            &public_url("resume.pdf"),
                            "Resume",
                            SIZE[13],
                        ))
                        .item(El::new().s(Align::center()).child("Human")),
                )
                .item(
                    Column::new()
                        .item(super::svg_link(
                            "icons/pdf.svg",
                            &public_url("ATS Resume.pdf"),
                            "ATS Resume",
                            SIZE[13],
                        ))
                        .item(El::new().s(Align::center()).child("ATS")),
                ),
        ),
    )
}

fn call_to_action() -> impl Element {
    super::section(TRANSPARENT, SPACING[6], El::new().child("Work in progress on the entire site. Built in a Rust framework MoonZoon so development is a bit slow."))
}
