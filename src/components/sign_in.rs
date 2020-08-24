use yew::prelude::*;

pub struct SignIn {
    email: String,
    password: String,
    link: ComponentLink<Self>,
}

pub enum Msg {
    EmailChange(String),
    PasswordChange(String),
}

impl Component for SignIn {
    type Properties = ();
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::EmailChange(email) => self.email = email,
            Msg::PasswordChange(password) => self.password = password,
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let email_change = self.link.callback(|data: ChangeData| {
            Msg::EmailChange(match data {
                ChangeData::Value(value) => value,
                _ => unreachable!(),
            })
        });
        let password_change = self.link.callback(|data: ChangeData| {
            Msg::PasswordChange(match data {
                ChangeData::Value(value) => value,
                _ => unreachable!(),
            })
        });
        html! {
            <div class="sign-in grid">
                <fieldset class="sign-in__form">
                    <legend>{"Sign In"}</legend>
                    <input type="text" placeholder="Email" onchange=email_change />
                    <input type="password" placeholder="Password" onchange=password_change />
                </fieldset>
            </div>
        }
    }
}
