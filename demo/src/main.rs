#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

type Point = [f64; 2];

#[component]
fn App() -> Element {
    let mut points = use_signal(Vec::<Point>::new);

    let smallest_circle =
        smallest_enclosing_circle::smallest_enclosing_circle((points.clone())().into_iter());
    let center = smallest_circle.center();
    let radius = smallest_circle.radius();

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        svg {
            width: "600",
            height: "600",
            "style": "background-color: #fff",
            onclick: move |event| {
                let point = event.data.element_coordinates();
                points.push([point.x, point.y]);
            },
            if center.is_some() {
                circle {
                    cx: center.unwrap()[0],
                    cy: center.unwrap()[1],
                    r: radius,
                    class: "smallestCircle"
                }
            }
            {points.iter().enumerate().map(|(i, point)| rsx!{
                circle {
                    cx: point[0],
                    cy: point[1],
                    r: "2%",
                    class: format!("plainPoint {}", if smallest_circle.is_spanned_by(&point) { "spanningPoint" } else { "" }),
                    onclick: move |event| {
                        event.stop_propagation();
                        points.remove(i);
                    }
                }
            })}
        }
    }
}
