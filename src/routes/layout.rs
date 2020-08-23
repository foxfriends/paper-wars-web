use super::AppRoute;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::components::Nav;

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
            <>
                <div class="layout grid">
                    <div class="layout__logo">
                        {"Paper Wars"}
                    </div>
                    <header class="layout__header">
                        // TODO: header
                    </header>
                    <nav class="layout__nav">
                        <Nav />
                    </nav>
                    <main class="layout__main">
                        { self.props.children.clone() }
                    </main>
                    <aside class="layout__aside">
                        // TODO: aside
                    </aside>
                    <footer class="layout__footer">
                        // TODO: footer
                    </footer>
                </div>
            </>
        }
    }
}
