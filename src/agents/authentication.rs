use super::CLIENT;
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;
use yew::worker::*;

#[derive(Clone, Debug)]
pub enum Request {
    Authenticate {
        email: String,
        password: String,
    },
    Reauthenticate,
}

pub struct Authentication {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for Authentication {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = bool;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link, subscribers: HashSet::new() }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, req: Self::Input, _: HandlerId) {
        let link = self.link.clone();
        let subscribers = self.subscribers.clone();
        spawn_local(async move {
            let result = match req {
                Request::Authenticate { email, password } => CLIENT.authenticate(email, password).await,
                Request::Reauthenticate => {
                    if let Some(token) = auth_token() {
                        CLIENT.reauthenticate(token).await
                    } else {
                        Err(anyhow::anyhow!("No token available"))
                    }
                }
            };
            match result {
                Ok(token) => {
                    set_auth_token(Some(&token));
                    subscribers.into_iter().for_each(|id| link.respond(id, true))
                }
                Err(..) =>
                    subscribers.into_iter().for_each(|id| link.respond(id, false)),
            }
        });
    }

    fn connected(&mut self, id: HandlerId) {
        let link = self.link.clone();
        self.subscribers.insert(id);
        spawn_local(async move {
            link.respond(id, CLIENT.is_authenticated().await)
        });
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

const STORAGE_KEY: &str = "authtoken";

fn auth_token() -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()??
        .get_item(STORAGE_KEY)
        .ok()?
}

fn set_auth_token(token: Option<&str>) {
    let local_storage = web_sys::window()
        .and_then(|window| window.local_storage().ok()?);
    let local_storage = match local_storage {
        Some(local_storage) => local_storage,
        None => return,
    };
    match token {
        Some(token) => local_storage.set_item(STORAGE_KEY, token).ok(),
        None => local_storage.remove_item(STORAGE_KEY).ok(),
    };
}
