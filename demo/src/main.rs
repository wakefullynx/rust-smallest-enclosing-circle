#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(App);
}

type Point = [f64; 2];

const INITIAL_POINTS: [Point; 5] = [[200., 200.], [500., 600.], [200., 500.], [400., 400.], [600., 300.]];

#[component]
fn App() -> Element {
    let mut points = use_signal(|| Vec::from(INITIAL_POINTS));

    let smallest_circle =
        smallest_enclosing_circle::smallest_enclosing_circle((points)().into_iter());
    let center = smallest_circle.center();
    let radius = smallest_circle.radius();

    rsx! {
        title { "Test" }
        link { rel: "stylesheet", href: "main.css" }
        svg {
            width: "100%",
            height: "100vh",
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
                    r: "1%",
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
