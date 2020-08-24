use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod agents;
mod components;
mod routes;

use agents::{authentication, Authentication};
use components::SignIn;
use routes::Routes;

pub enum Msg {
    SetIsAuthenticated(bool),
}

pub struct Root {
    _authentication: Box<dyn Bridge<Authentication>>,
    is_authenticated: Option<bool>,
    link: ComponentLink<Self>,
}

impl Component for Root {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut authentication = Authentication::bridge(link.callback(Msg::SetIsAuthenticated));
        authentication.send(authentication::Request::Reauthenticate);
        Self {
            _authentication: authentication,
            is_authenticated: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetIsAuthenticated(is_authenticated) => {
                self.is_authenticated = Some(is_authenticated)
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.is_authenticated {
            Some(true) => html! { <Routes /> },
            Some(false) => html! { <SignIn /> },
            None => html! { <></> },
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}
