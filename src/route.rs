use yew_router::Routable;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/display/:data")]
    Display { data: String },
}
