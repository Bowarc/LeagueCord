pub struct Contact;

impl yew::Component for Contact {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use yew::html;

        html! {<>
            <div class="contact">
                <h1>{"Contact Me"}</h1>
                <p>{"You can reach me at: "}</p>
                <table>
                    <tr>
                        <th>{
                            "Email: "
                        }</th>
                        <td>{
                            "contact@bowarc.ovh"
                        }</td>
                    </tr>
                </table>
                
            </div>
        </>}
    }
}
