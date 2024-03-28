use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;

use crate::{api::converse, app::components::{chat_area::ChatArea, type_area::TypeArea}, model::conversation::{Conversation, Message}};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rusty-ai-chat.css"/>

        // sets the document title
        <Title text="Rusty AI Chat"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (conversation, set_conversation) = create_signal(Conversation::new());

    let send = create_action(move | new_message: &String | {
        let user_message = Message {
            user: true,
            text: new_message.clone(),
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        converse(conversation.get())
    });

    // Add loading effect
    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                user: false,
                text: "Loading...".to_string(),
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    // Handle the response from the server
    create_effect(move |_|{
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                // Update the last message ("Loading ...") with the response
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });


    view! {
        <ChatArea conversation/>
        <TypeArea send/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
