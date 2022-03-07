use std::rc::Rc;

use yew::prelude::*;
use yew_chart::{
    axis::AxisScale,
    horizontal_axis::{self, HorizontalAxis},
    linear_axis_scale::{Labeller, LinearAxisScale},
    vertical_axis::{self, VerticalAxis},
};

const WIDTH: f32 = 533.0;
const HEIGHT: f32 = 150.0;
const MARGIN: f32 = 30.0;
const TICK_LENGTH: f32 = 10.0;

#[function_component(App)]
fn app() -> Html {
    fn empty_labeller() -> Box<Labeller> {
        Box::new(move |_| "".to_string())
    }

    let h_scale = Rc::new(LinearAxisScale::with_labeller(
        0.0..5.0,
        1.0,
        Rc::new(empty_labeller()),
    )) as Rc<dyn AxisScale>;
    let v_scale = Rc::new(LinearAxisScale::new(0.0..5.0, 1.0)) as Rc<dyn AxisScale>;

    html! {
        <>
            <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
                <VerticalAxis
                    name="some-y-axis"
                    orientation={vertical_axis::Orientation::Left}
                    scale={Rc::clone(&v_scale)}
                    x1={MARGIN} y1={MARGIN} y2={HEIGHT - TICK_LENGTH}
                    tick_len={TICK_LENGTH}
                    title={"Some Y thing".to_string()} />
                <HorizontalAxis
                    name="some-x-axis"
                    orientation={horizontal_axis::Orientation::Bottom}
                    scale={Rc::clone(&h_scale)}
                    x1={MARGIN} y1={HEIGHT - TICK_LENGTH} x2={WIDTH}
                    tick_len={TICK_LENGTH} />
            </svg>
            <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, TICK_LENGTH)} preserveAspectRatio="none">
                <HorizontalAxis
                    name="some-x-axis"
                    orientation={horizontal_axis::Orientation::Bottom}
                    scale={Rc::clone(&h_scale)}
                    x1={MARGIN} y1={0.0} x2={WIDTH}
                    tick_len={TICK_LENGTH} />
            </svg>
            <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
                <HorizontalAxis
                    name="some-x-axis"
                    orientation={horizontal_axis::Orientation::Top}
                    scale={Rc::clone(&h_scale)}
                    x1={MARGIN} y1={TICK_LENGTH} x2={WIDTH}
                    tick_len={TICK_LENGTH} />
                <VerticalAxis
                    name="some-y-axis"
                    orientation={vertical_axis::Orientation::Left}
                    scale={Rc::clone(&v_scale)}
                    x1={MARGIN} y1={TICK_LENGTH} y2={HEIGHT - MARGIN}
                    tick_len={TICK_LENGTH}
                    title={"Some Y thing".to_string()} />
            </svg>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
