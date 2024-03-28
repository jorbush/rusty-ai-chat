use leptos::*;

#[component]
pub fn TypeArea(send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<leptos::html::Input>();
    view! {
        <div class="h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t bg-white border-gray-300">
            <form on:submit= move |ev| {
                ev.prevent_default(); // prevent the form from submitting
                let input = input_ref.get().expect("input element not found");
                send.dispatch(input.value());
                input.set_value("");
            }>
                <input type="text" class="w-2/3 p-4 border border-gray-300 rounded-full input-field" node_ref=input_ref/>
                <input type="submit" class="h-full p-4 bg-blue-50 text-white rounded-full cursor-pointer"/>
            </form>
        </div>
    }
}
