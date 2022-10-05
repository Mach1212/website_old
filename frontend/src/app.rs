use crate::web_sys::HtmlAnchorElement;
use zoon::button::OnPressFlagNotSet;
use zoon::column::EmptyFlagNotSet;
use zoon::el::ChildFlagSet;
use zoon::web_sys::{HtmlDivElement, HtmlElement};
use zoon::FontWeight::{Bold, Medium};
use zoon::*;

mod home;
mod projects;

/***************************************/
/* Routing                             */
/***************************************/

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    Home,
    Projects,
    Unknown,
}

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Unknown)
}

#[static_ref]
fn dark_mode() -> &'static Mutable<bool> {
    Mutable::new(false)
}

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

fn page() -> impl Element {
    El::new().child_signal(page_id().signal().map(|page_id| match page_id {
        PageId::Home => home::page().into_raw_element(),
        PageId::Projects => projects::page().into_raw_element(),
        PageId::Unknown => El::new().child("404").into_raw_element(),
    }))
}

/***************************************/
/* Constants                           */
/***************************************/

pub const BACKGROUND: [HSLuv; 3] = [
    // #f8f9fa
    hsluv! {235.5, 18.4, 97.9},
    // #f1f3f5
    hsluv! {235.7, 18.3, 95.7},
    // #e9ecef
    hsluv! {235.8, 17.2, 93.3},
];

pub const CORNER_RADIUS: u32 = 9;

pub const PAGE_WIDTH: u32 = 1500;
// #edf2ff
pub const ACCENT_TINT: HSLuv = hsluv! {252.3, 100.0, 95.5};
// #4263eb
pub const ACCENT: HSLuv = hsluv! {261.9, 87.7, 47.1};
// #364fc7
pub const ACCENT_SHADE: HSLuv = hsluv! {262.5, 79.0, 38.7};
// #343a40 #868e96
pub const TEXT: [HSLuv; 2] = [hsluv! {237.8, 19.6, 24.1}, hsluv! {236.9, 12.3, 58.6}];

// Minor third with 10pt base
pub const SIZE: [u32; 15] = [10, 12, 14, 16, 18, 20, 24, 30, 36, 44, 52, 62, 74, 86, 98];
pub const SPACING: [u32; 12] = [2, 4, 8, 12, 16, 24, 32, 48, 64, 80, 96, 128];

/***************************************/
/* Global styles                         */
/***************************************/

pub fn root() -> impl Element {
    Column::new()
        .item(section(ACCENT_TINT, 0, header()))
        .item(page())
    // .item(footer())
}

fn header<'a>() -> impl Element + Styleable<'a> {
    Row::new()
        .s(Font::new()
            .size(SIZE[5])
            .weight(FontWeight::SemiBold)
            .color(ACCENT))
        .s(Padding::new().top(SPACING[3]).bottom(SPACING[3]))
        .item(
            Link::new()
                .label(
                    El::new()
                        .s(Font::new().size(SIZE[8]).color(ACCENT))
                        .child("MP"),
                )
                .to("home"),
        )
        .item(Spacer::fill())
        .item(
            Row::new()
                .s(Gap::new().x(SPACING[7]))
                .item(Link::new().label("Home").to("home"))
                .item(Link::new().label("Projects").to("projects")),
        )
        .item(Spacer::fill())
        .item(
            Link::new()
                .label(make_button(
                    "Email Me",
                    ACCENT_TINT,
                    ACCENT,
                    ACCENT_SHADE,
                    || {},
                ))
                .to("mailto:mpruchn@ncsu.edu")
                .new_tab(NewTab::new().follow(true)),
        )
}

// fn footer() -> impl Element {}

/***************************************/
/* Components                          */
/***************************************/
pub fn section<'a, T: Element + Styleable<'a>>(
    background: HSLuv,
    top_padding: u32,
    child: T,
) -> El<ChildFlagSet, RawHtmlEl<HtmlElement>> {
    El::new()
        .s(Width::fill())
        .s(Background::new().color(background))
        .s(Padding::new().x(SPACING[2]).top(top_padding))
        .child(child.s(Width::fill().max(PAGE_WIDTH)).s(Align::center()))
}

pub fn make_button(
    label: &str,
    color: HSLuv,
    background: HSLuv,
    background_active: HSLuv,
    on_click: impl FnMut() + 'static,
) -> Button<button::LabelFlagSet, OnPressFlagNotSet, RawHtmlEl<HtmlDivElement>> {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .label(label)
        .s(Padding::new().x(16).y(8))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(move || background_active, move || background)))
        .s(RoundedCorners::all(9))
        .s(Font::new().color(color).no_wrap())
        .s(Transitions::new([
            Transition::background_color().duration(500)
        ]))
        .on_hovered_change(move |change| hovered.set_neq(change))
        .on_click(on_click)
}

pub fn make_svg_link(
    label: &str,
    link: &str,
    width: u32,
) -> Link<link::LabelFlagSet, link::ToFlagSet, RawHtmlEl<HtmlAnchorElement>> {
    Link::new()
        .label(
            Row::new().item(label).item(
                Image::new()
                    .url(public_url("icons/out_box.svg"))
                    .description("icon")
                    .s(Width::exact(width)),
            ),
        )
        .to(link)
}

pub fn svg_link(
    svg_url: &str,
    link: &str,
    description: &str,
    width: u32,
) -> Link<link::LabelFlagSet, link::ToFlagSet, RawHtmlEl<HtmlAnchorElement>> {
    Link::new()
        .label(svg(svg_url, description, width))
        .to(link)
        .new_tab(NewTab::new().follow(true))
}

pub fn svg(svg_url: &str, description: &str, width: u32) -> impl Element {
    Image::new()
        .s(Width::exact(width))
        .url(public_url(svg_url))
        .description(description)
}

pub fn h2(text: &str) -> impl Element {
    El::new()
        .child(text)
        .s(Font::new().color(ACCENT).size(SIZE[10]))
}
