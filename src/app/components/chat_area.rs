use::leptos::*;

use crate::model::conversation::Conversation;

const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500 text-white";
const AI_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start bg-gray-200 text-black";

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    view! {
        <div>
            {move || conversation.get().messages.iter().map(move |message| {
                let class_name = if message.user {
                    USER_MESSAGE_CLASS
                } else {
                    AI_MESSAGE_CLASS
                };
                view! {
                    <div class={class_name}>
                        {message.text.clone()}
                    </div>
                }
            }).collect::<Vec<_>>()
        }
        </div>
    }
}
