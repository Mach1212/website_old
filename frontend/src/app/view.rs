use crate::app::{counter, increment};
use zoon::named_color::{GRAY_0, GREEN_7, GREEN_8};
use zoon::*;

pub fn root() -> impl Element {
    Row::new()
        .s(Align::center())
        .s(Transform::new().move_up(20))
        .s(Gap::both(20))
        .s(Font::new().color(GRAY_0).size(30))
        .item(increment_button())
        .item_signal(counter().signal())
}

fn increment_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::new().x(12).y(7))
        .s(RoundedCorners::all(10))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GREEN_7, || GREEN_8)))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Increment!")
        .on_press(increment)
}
