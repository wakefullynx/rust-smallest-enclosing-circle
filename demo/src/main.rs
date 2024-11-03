#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{BsBoxes, BsGithub};
use dioxus_free_icons::Icon;

fn main() {
    launch(App);
}

type Point = [f64; 2];

const INITIAL_POINTS: [Point; 5] = [
    [400., 200.],
    [700., 600.],
    [400., 500.],
    [600., 400.],
    [800., 300.],
];

#[component]
fn App() -> Element {
    let mut points = use_signal(|| Vec::from(INITIAL_POINTS));

    let smallest_circle =
        smallest_enclosing_circle::smallest_enclosing_circle((points)().into_iter());
    let center = smallest_circle.center();
    let radius = smallest_circle.radius();

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        header {
            h1 {
                "Smallest Enclosing Circle Demo"
            }
            p {
                a {
                    href: "https://github.com/wakefullynx/rust-smallest-enclosing-circle",
                    Icon {
                        class: "icon",
                        icon: BsGithub,
                    }
                    " GitHub"
                }
                a {
                    href: "https://crates.io/crates/smallest-enclosing-circle",
                    Icon {
                        class: "icon",
                        icon: BsBoxes,
                    }
                    " Crates.io"
                }
            }
        }
        div { class: "svg_parent",
            div {
                class: "overlay",
                p {
                    "Click anywhere to create points."
                }
            }
            svg {
                onclick: move |event| {
                let point = event.data.element_coordinates();
                    points.push([point.x, point.y]);
                },
                if center.is_some() {
                    circle {
                        cx: center.unwrap()[0],
                        cy: center.unwrap()[1],
                        r: radius,
                        class: "smallest_circle"
                    }
                }
                g {
                    {points.iter().enumerate().map(|(i, point)| rsx!{
                        circle {
                            key: "{i}",
                            cx: point[0],
                            cy: point[1],
                            r: "1%",
                            class: format!("plain_point {}", if smallest_circle.is_spanned_by(&point) { "spanning_point" } else { "" }),
                            onclick: move |event| {
                                event.stop_propagation();
                                points.remove(i);
                            }
                        }
                    })}
                }
            }
        }
    }
}
