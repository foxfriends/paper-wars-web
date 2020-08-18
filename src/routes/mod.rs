use yew::prelude::*;
use yew_router::prelude::*;

mod about;
mod index;

use about::About;
use index::Index;

#[derive(Clone, Debug, Switch)]
pub enum Route {
    #[to = "/about/"]
    About,
    #[to = "/"]
    Index,
}

pub struct Routes {
    link: ComponentLink<Self>,
}

impl Component for Routes {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<Route, ()>
                render = Router::render(|switch: Route|  match switch {
                    Route::Index => html! { <Index /> },
                    Route::About => html! { <About /> },
                })
            />
        }
    }
}
