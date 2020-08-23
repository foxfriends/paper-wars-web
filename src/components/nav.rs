use crate::routes::AppRoute;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct Nav;

impl Component for Nav {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <RouterAnchor<AppRoute> route=AppRoute::Index classes="nav__item">
                    {"Home"}
                </RouterAnchor<AppRoute>>
            </>
        }
    }
}
