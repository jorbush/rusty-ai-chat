// Hold all server-side functions
use leptos::*;
use crate::model::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(prompt: Conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::web::ConnectionInfo;

    let model = extract(|data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner();
    }).await.unwrap();

    use llm::KnownModel;
    let character_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and an AI assistant.";
    let history = format!(
        "{CHARACTER_NAME}:Hello - How may I help you today?\n\
        {USER_NAME}:What is the capital of France?\n\
        {CHARACTER_NAME}:Paris is the capital of France.\n"
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let current_line = if message.user {
            format!("{character_name}:{msg}\n")
        } else {
            format!("{user_name}:{msg}\n")
        };
        history.push_str(&current_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buffer = String::new();

    // Start a new session for each request
    let mut session = model.start_session(Default::default());

    session.infer(
        model.as_ref(),
        &mut rng,
        &llm::InferenceRequest {
            prompt: format!(
                "{persona}\n{history}\n{character_name}:"
            ).as_str().into(),
            parameters: Some(&llm::InferenceParameters::default()),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        }, // input request
        &mut Default::default(), // output response
        inference_callback(String::from(user_name), &mut buffer, &mut res),
    ).unwrap_or_else(|e| {
        panic!("Error: {}", e);
    });

    Ok(String::from(""))
}

// only compile in the server-side binary
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::convert::Infallible;
        /*
        This function needs to know when it should stop getting
        new tokens from the lenguage model inference.
        */
        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            out_str: &'a mut String,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
            use llm::InferenceFeedback::Halt;
            use llm::InferenceFeedback::Continue;

            move |resp| match resp {
                llm::InferenceResponse::InferredToken(t) => {
                    let mut reverse_buf = buf.clone();
                    reverse_buf.push_str(t.as_str());
                    if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                        buf.clear();
                        return Ok::<llm::InferenceFeedback, Infallible>(Halt);
                    } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                        buf.push_str(t.as_str());
                        return Ok(Continue);
                    }

                    if buf.is_empty() {
                        out_str.push_str(t.as_str());
                    } else {
                        out_str.push_str(reverse_buf.as_str());
                    }

                    Ok(Continue)
                }
                llm::InferenceResponse::EotToken => Ok(Halt),
                _ => Ok(Continue),
            }
        }
    }
}
