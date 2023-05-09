use wasm_bindgen::JsCast;
use web_sys::HtmlMediaElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <VideoFunction />
            //<VideoStruct />
        </main>
    }
}

#[function_component(VideoFunction)]
pub fn video_function() -> Html {
    let number = use_state_eq(|| None::<i64>);

    let time_update = {
        let number = number.clone();
        Callback::from(move |event| {
            number.set(number_from_video(event));
        })
    };

    let numbers = {
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

    let number_opt = number.map(|n| html! {
        <button class="my-button">
            {format!("Number: {n}")}
        </button>
    });

    html! {
        <>
            <video autoplay=true type={VIDEO_TYPE} src={VIDEO_URL}
                ontimeupdate={time_update}>
            </video>

            //{ numbers } // Bug
            //{ for number_opt } // Bug

            // Works
            if let Some(num) = *number {
                <button class="my-button">
                    {format!("Number: {num}")}
                </button>
            }
        </>
    }
}

#[derive(Default)]
struct VideoStruct {
    number: Option<i64>,
}

impl Component for VideoStruct {
    type Message = Option<i64>;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        VideoStruct::default()
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        if self.number == msg {
            false
        } else {
            self.number = msg;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let numbers = {
            if let Some(num) = &self.number {
                vec![html! {
                    <button class="my-button">
                        {format!("Number: {num}")}
                    </button>
                }]
            } else {
                Vec::new()
            }
        };

        let number_opt = self.number.map(|n| html! {
            <button class="my-button">
                {format!("Number: {n}")}
            </button>
        });

        html! {
            <>
                <video autoplay=true type={VIDEO_TYPE} src={VIDEO_URL}
                    ontimeupdate={ctx.link().callback(number_from_video)}
                ></video>

                //{ numbers } // Bug
                { for number_opt } // Bug

                // Works
                // if let Some(num) = &self.number {
                //     <button class="my-button">
                //         {format!("Number: {num}")}
                //     </button>
                // }
            </>
        }
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
