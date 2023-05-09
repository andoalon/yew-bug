mod app;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render(); // yew 0.20.0
    //yew::start_app::<app::App>(); // yew 0.19.3
}
