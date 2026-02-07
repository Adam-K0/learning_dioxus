use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let mut img = use_signal(|| "https://upload.wikimedia.org/wikipedia/commons/thumb/b/b0/View_west_along_Duomo_roof%2C_Milan.jpg/960px-View_west_along_Duomo_roof%2C_Milan.jpg");

    let next_pic = move |_| {
        img.set("https://upload.wikimedia.org/wikipedia/commons/thumb/5/52/Milano_-_Torre_del_Filarete_-_2024-09-02_21-41-46_001.jpg/500px-Milano_-_Torre_del_Filarete_-_2024-09-02_21-41-46_001.jpg");
    };

    let prev_pic = move |_| {
        img.set("https://upload.wikimedia.org/wikipedia/commons/thumb/b/b0/View_west_along_Duomo_roof%2C_Milan.jpg/960px-View_west_along_Duomo_roof%2C_Milan.jpg");
    };

    rsx! {
        document::Stylesheet { href: CSS }

        div { id: "title",
            h1 { "Milan"}
        }

        div { id: "pic_view",
            img { src: "{img}" }
        }

        div { class: "buttons",
            button { onclick: next_pic, id: "next", "Next" }
            button { onclick: prev_pic, id: "prev", "Prev" }
        }
    }
}

#[component]
fn DogApp(breed: String) -> Element {
    rsx! {
        "Breed: {breed}"
    }
}
