use leptos::{component, create_signal, event_target_value, IntoView, view};

#[component]
pub  fn Login() -> impl IntoView{
    let (email, set_email) = create_signal("".to_string());
   view! {
       <div>
           <div class="m-5">
               <label for="email_input" class="form-label">Username:</label>
               <input
                   class="form-control"
                   id="email_input"
                   type="email"
                    placeholder="name@email.com"
                   on:input=move |x| {
                       set_email(event_target_value(&x));
                   }

                   prop:value=email
               />
           </div>
       </div>
   } 
}