use super::AppRoute;
use crate::services::CLIENT;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Layout {
    props: Props,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub route: AppRoute,
    pub children: Children,
}

impl Component for Layout {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <div class="layout">
                <header class="layout__header">
                    // TODO: header
                </header>
                <main class="layout__main">
                </main>
                <footer class="layout__footer">
                    // TODO: footer
                </footer>
            </div>
        }
    }
}
