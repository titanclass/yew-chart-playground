use std::rc::Rc;

use yew::prelude::*;
use yew_chart::{
    axis::{Axis, Orientation, Scale},
    linear_axis_scale::LinearScale,
};

const WIDTH: f32 = 533.0;
const HEIGHT: f32 = 150.0;
const MARGIN: f32 = 30.0;
const TICK_LENGTH: f32 = 10.0;

#[function_component(App)]
fn app() -> Html {
    let h_scale_top = Rc::new(LinearScale::new(0.0..5.0, 1.0)) as Rc<dyn Scale>;
    let h_scale_bottom = Rc::new(LinearScale::with_labeller(0.0..5.0, 1.0, None)) as Rc<dyn Scale>;
    let v_scale = Rc::new(LinearScale::new(0.0..5.0, 1.0)) as Rc<dyn Scale>;

    html! {
        <>
        <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
            <Axis
                name="some-y-axis"
                orientation={Orientation::Left}
                scale={Rc::clone(&v_scale)}
                x1={MARGIN} y1={MARGIN} xy2={HEIGHT - TICK_LENGTH * 2.0}
                tick_len={TICK_LENGTH}
                title={"Some Y thing".to_string()} />
            <Axis
                name="some-x-axis"
                orientation={Orientation::Bottom}
                scale={Rc::clone(&h_scale_top)}
                x1={MARGIN} y1={HEIGHT - TICK_LENGTH * 2.0} xy2={WIDTH - MARGIN}
                tick_len={TICK_LENGTH} />
        </svg>
        <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
            <Axis
                name="some-x-axis"
                orientation={Orientation::Top}
                scale={Rc::clone(&h_scale_bottom)}
                x1={MARGIN} y1={TICK_LENGTH} xy2={WIDTH - MARGIN}
                tick_len={TICK_LENGTH} />
            <Axis
                name="some-y-axis"
                orientation={Orientation::Left}
                scale={Rc::clone(&v_scale)}
                x1={MARGIN} y1={TICK_LENGTH} xy2={HEIGHT - MARGIN + TICK_LENGTH}
                tick_len={TICK_LENGTH}
                title={"Some Y thing".to_string()} />
        </svg>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
