use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
fn App(increment: i32) -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
    <div class="container">

            <picture>
                <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg" media="(prefers-color-scheme: dark)" />
                <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo" height="200" width="400"/>
            </picture>

        <h1>"Welcome to Leptos"</h1>
        <h2><i>"On Github Pages"</i></h2>

        <button
            on:click= move |_| {
                set_count(count() + increment)
            }
        >
            "Click me: "
            {count}
        </button>


    </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {

        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="gio"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router base="deploy_leptos_csr_to_gh_pages">
			<nav>
			  <A href="home">"Home"</A>"-"
			  <A href="about">"About"</A>"-"
			  <A href="contact">"Contact"</A>"-"
			</nav>
        <App increment=5 />
            <Routes base="deploy_leptos_csr_to_gh_pages".to_owned()>
                <Route path="" view=Home/>
                <Route path="/" view=Home/>
                <Route path="/home" view=Home/>
                <Route path="/about" view=About/>
                <Route path="/contact" view=Contact/>
                <Route path="/*" view=NotFound/>
            </Routes>
        </Router>
        }
    })
}

#[component]
pub fn About() -> impl IntoView {
    view! { <h1>"About"</h1>  }
}

#[component]
pub fn Contact() -> impl IntoView {
    view! { <h1>"Contact"</h1>  }
}

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! { <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>  }
}


/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to my site!"</h1>
                <p>"A site written in Rust using "<a href="https://github.com/leptos-rs/leptos" target="_blank">"Leptos"</a>" deployed on Github Pages"</p>
                <p>"The backend will also be written in Rust and deployed on AWS Lambda"</p>
            </div>
        </ErrorBoundary>
    }
}
