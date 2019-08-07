use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod constants;

struct Model {}

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                true
            }
        }
    }
}

fn row_item(text: &str) -> Html<Model> {
    html! {
        <td> { text } </td>
    }
}

fn rows() -> Html<Model> {
    html! {
        <tr>
            {
                for (0..constants::ITEMS.len()).map(|item| {
                    row_item(constants::ITEMS[item])
                })
            }
        </tr>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <table>
                {
                    rows()
                }
            </table>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}