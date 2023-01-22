use std::rc::Rc;

use patternfly_yew::Popover;
use patternfly_yew::{Color, Label};
use yew::{function_component, html, Html, Properties};

use crate::error::FrontendError;

#[derive(Properties, PartialEq)]
pub struct ErrorProps {
    pub error: Rc<FrontendError>,
}

#[function_component(Error)]
pub fn error(e: &ErrorProps) -> Html {
    let target = html! {<Label color={Color::Red} label={e.error.to_string()}/>};
    let content = e.error.format_error();

    html! {<Popover {target} toggle_by_onclick=true>{content}</Popover>}
}
