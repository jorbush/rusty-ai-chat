use::leptos::*;

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    view! {
        <div>
            {move || conversation.get().messages.iter().map(move |message| {
                view! {
                    <div>
                        {message.text.clone()}
                    </div>
                }
            }).collect::<Vec<_>>()
        }
        </div>
    }
}
