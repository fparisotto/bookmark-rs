use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::api::search_api::SearchType;
use crate::components::atoms::input_text::{InputText, InputType};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct SearchInputSubmit {
    pub input: String,
    pub search_type: SearchType,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub email: String,
    pub add_new_bookmark_modal_id: String,
    pub on_submit: Callback<SearchInputSubmit>,
}

#[function_component(NavigationBar)]
pub fn navigation_bar(props: &Props) -> Html {
    let state = use_state(SearchInputSubmit::default);

    let modal_id = format!("#{}", &props.add_new_bookmark_modal_id);

    let on_input_search_change = {
        let state = state.clone();
        Callback::from(move |text: String| {
            let mut data = (*state).clone();
            data.input = text;
            state.set(data);
        })
    };

    let on_submit = {
        let on_search = props.on_submit.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let data = (*state).clone();
            on_search.emit(data);
        })
    };

    html! {
        <nav class="col-span-6 navbar">
            <a class="btn btn-ghost uppercase text-lg">{"Bookmarks"}</a>
            <form onsubmit={on_submit} class="flex-1 justify-end gap-2">
                <div class="input-group flex-1 justify-end">
                    <InputText
                        id="search"
                        name="search"
                        placeholder="search..."
                        class={classes!("input", "input-bordered")}
                        input_type={InputType::Search}
                        on_change={on_input_search_change} />
                    <button class="btn btn-square">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                        </svg>
                    </button>
                </div>
                <a href={modal_id} class="flex-none btn btn-sm btn-accent">{"+new"}</a>
            </form>
            <a class="btn btn-ghost normal-case ">{"E-mail: "} {&props.email}</a>
        </nav>
    }
}
