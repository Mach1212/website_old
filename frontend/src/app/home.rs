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
}

fn introduction() -> impl Element {
    super::section(
        ACCENT_TINT,
        0,
        Row::new()
            .s(Padding::new().bottom(SPACING[5]))
            .s(Gap::new().x(SPACING[1]))
            .item(
                Column::new()
                    .s(Gap::both(SPACING[5]))
                    .item(
                        El::new()
                            .s(Width::fill().max(800))
                            .s(Font::new().size(SIZE[12]).line_height(SPACING[9]))
                            .child("Hi, I am Maciej Pruchnik, a software engineer"),
                    )
                    .item(
                        El::new()
                            .child("Solving your problems with elegant solutions")
                            .s(Font::new().size(SIZE[5]).italic()),
                    )
                    .item(
                        Link::new()
                            .label(super::button(
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
        Column::new()
            .s(Gap::both(SPACING[4]))
            .item(super::h2("Education"))
            .item(education_item(
                "ncsu.png",
                90,
                "2020-08 - 2024-05",
                "BA in Computer Science",
                "North Carolina State University, Raleigh, North Carolina",
                ["Software Engineering(Java), Data Structures(Java), C Programming"].into_iter(),
            ))
            .item(education_item(
                "waketech.png",
                200,
                "2019-08 - 2020-05",
                "Transfer to NCSU",
                "Waketech, Raleigh, North Carolina",
                ["Public Speaking", "Deans list in Fall 2019"].into_iter(),
            ))
            .item(education_item(
                "adhs.png",
                100,
                "2015-08 - 2019-05",
                "GED",
                "Athens Drive Magnet High, Raleigh, North Carolina",
                [
                    "AP Computer Science",
                    "Graduated Cum Laude",
                    "In the award winning marching band for four years and a section leader for one",
                    "Co-founded a drone club",
                ]
                .into_iter(),
            )),
    )
}

fn education_item(
    logo_url: &str,
    width: u32,
    duration: &str,
    title: &str,
    location: &str,
    achievements: impl Iterator<Item = &'static str>,
) -> Row<EmptyFlagNotSet, MultilineFlagNotSet, RawHtmlEl<HtmlElement>> {
    Row::new()
        .s(Background::new().color(ACCENT_TINT))
        .s(RoundedCorners::all(CORNER_RADIUS))
        .s(Padding::new().x(SPACING[7]).y(SPACING[6]))
        .s(Gap::both(SPACING[6]))
        .item(
            Column::new()
                .s(Align::new().top())
                .item(El::new().child(duration).s(Font::new().no_wrap()))
                .item(
                    Image::new()
                        .s(Width::growable().max(width))
                        .s(Align::center())
                        .url(public_url(logo_url))
                        .description(&(location.to_string() + " logo"))
                        .update_raw_el(|el| el.style("filter", "grayscale(100)")),
                ),
        )
        .item(
            Column::new()
                .s(Align::new().top())
                .item(
                    El::new()
                        .s(Font::new().color(ACCENT).weight(FontWeight::SemiBold))
                        .child(title),
                )
                .item(El::new().s(Font::new().italic()).child(location))
                .item(
                    Column::new()
                        .s(Padding::new().top(SPACING[1]))
                        .items(achievements.map(|achievement| {
                            Row::new()
                                .s(Gap::both(SPACING[1]))
                                .item(
                                    El::new()
                                        .s(Padding::new().left(SPACING[2]))
                                        .s(Align::new().top())
                                        .child("•"),
                                )
                                .item(El::new().child(achievement))
                        })),
                ),
        )
}

fn experience() -> impl Element {
    super::section(
        TRANSPARENT,
        SPACING[4],
        Column::new()
            .s(Gap::both(SPACING[4]))
            .item(super::h2("Experience"))
            .item(experience_item(
                "tizbi.png",
                150,
                "2019-06 - 2020-02",
                "Webdev Intern",
                "Tizbi, Raleigh, North Carolina",
                ["Created static webpages using NextJS, a Javascript/Typescript full-stack framework, for a business expansion called ForceComprehensive", "Developed an Android app as an internal tool for FlameOffCoating's sales team to estimate paint requirements"].into_iter(),
            )),
    )
}

fn experience_item(
    logo_url: &str,
    width: u32,
    duration: &str,
    title: &str,
    location: &str,
    achievements: impl Iterator<Item = &'static str>,
) -> Row<EmptyFlagNotSet, MultilineFlagNotSet, RawHtmlEl<HtmlElement>> {
    Row::new()
        .s(Background::new().color(ACCENT_TINT))
        .s(RoundedCorners::all(CORNER_RADIUS))
        .s(Padding::new().x(SPACING[7]).y(SPACING[6]))
        .s(Gap::both(SPACING[6]))
        .item(
            Column::new()
                .s(Align::new().top())
                .item(El::new().child(duration).s(Font::new().no_wrap()))
                .item(
                    El::new().child(
                        Image::new()
                            .s(Width::fill().max(width))
                            .s(Align::center())
                            .url(public_url(logo_url))
                            .description(&(location.to_string() + " logo"))
                            .update_raw_el(|el| el.style("filter", "grayscale(100)")),
                    ),
                ),
        )
        .item(
            Column::new()
                .s(Align::new().top())
                .item(
                    El::new()
                        .s(Font::new().color(ACCENT).weight(FontWeight::SemiBold))
                        .child(title),
                )
                .item(El::new().s(Font::new().italic()).child(location))
                .item(
                    Column::new()
                        .s(Padding::new().top(SPACING[1]))
                        .items(achievements.map(|achievement| {
                            Row::new()
                                .s(Gap::both(SPACING[1]))
                                .item(
                                    El::new()
                                        .s(Padding::new().left(SPACING[2]))
                                        .s(Align::new().top())
                                        .child("•"),
                                )
                                .item(El::new().child(achievement))
                        })),
                ),
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
                    .item(super::svg("icons/css.png", "CSS", SIZE[13]))
                    .item(super::svg("icons/c.png", "C", SIZE[13])),
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
