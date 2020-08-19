use yew::prelude::*;
use yew_router::prelude::*;

mod layout;
use layout::Layout;

mod about;
mod index;
use about::About;
use index::Index;

#[derive(Clone, Eq, PartialEq, Debug, Switch)]
pub enum AppRoute {
    #[to = "/about/"]
    About,
    #[to = "/"]
    Index,
}

pub struct Routes;

impl Component for Routes {
    type Message = ();
    type Properties = ();

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
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| html! {
                    <Layout route=switch.clone()>
                        {
                            match switch {
                                AppRoute::Index => html! { <Index /> },
                                AppRoute::About => html! { <About /> },
                            }
                        }
                    </Layout>
                })
            />
        }
    }
}
