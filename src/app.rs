use wasm_bindgen::JsCast;
use web_sys::HtmlMediaElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Video />
        </main>
    }
}

#[function_component(Video)]
pub fn video() -> Html {
    let number = use_state_eq(|| None::<i64>);

    let time_update = {
        let number = number.clone();
        Callback::from(move |event| {
            number.set(number_from_video(event));
        })
    };

    let number_vec = {
        if let Some(num) = *number {
            vec![html! {
                <button class="my-button">
                    {format!("Number: {num}")}
                </button>
            }]
        } else {
            Vec::new()
        }
    };

    let number_opt = number.map(|num| html! {
        <button class="my-button">
            {format!("Number: {num}")}
        </button>
    });

    html! {
        <>
            <video muted=true controls=true type={VIDEO_TYPE} src={VIDEO_URL}
                ontimeupdate={time_update}>
            </video>

            // Case 1: Works
            // if let Some(num) = *number {
            //     <button class="my-button">
            //         {format!("Number: {num}")}
            //     </button>
            // }

            { number_vec } // Case 2: Bug
            //{ for number_vec } // Case 2.1: Bug

            //{ number_opt } // Case 3: Bug
            //{ for number_opt } // Case 3.1: Bug
        </>
    }
}

const VIDEO_TYPE: &str = "video/mp4";
const VIDEO_URL: &str = "/video.mp4";

fn number_from_video(event_from_video: Event) -> Option<i64> {
    let video = event_from_video
        .target()
        .unwrap()
        .dyn_into::<HtmlMediaElement>()
        .unwrap();

    let time = video.current_time() as i64;
    if time % 15 >= 7 {
        Some(time)
    } else {
        None
    }
}
