use yew::prelude::*;
use yew_router::prelude::*;
mod api;
mod component;
mod controller;
mod pages;
mod router;
mod constants {
    pub const COMPUTER_NAME: &str = "AI";
}
use router::{switch, Route};
// mod cell;
// mod cell_toot;
// mod connect4_computer;
// mod connect_4;
// mod MainPage;

// mod toot_otto_computer;
//implement yew router

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
        <div class="row">
            <div class ="side-bar">
            <SideNav/>

            </div>
            <div class="w3-mpub ain" style="margin-left:390px;margin-right:40px">
            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main></div>
        </div>
        </BrowserRouter>

    }
}

#[function_component(SideNav)]
fn side_nav() -> Html {
    html! {
        <div>
        <nav class="w3-sidenav w3-sand w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav"><br/>
                <div class="w3-container">
                  <h3 class="w3-padding-64"><b>{"Play"}<br/>{"Connect4 / TOOT-OTTO"}</b></h3>
                  <Link<Route>  to={Route::MainPage}>
                  { "Home" }
              </Link<Route>>
                </div>
                <Link<Route>  to={Route::HowToPlayConnect4}>
                                { "How to Play Connect4" }
                            </Link<Route>>
                            <Link<Route>  to={Route::Connect4}>
                            { "play connect4 vs human" }
                        </Link<Route>>
                        <Link<Route>  to={Route::Connect4Computer}>
                            { "play connect4 vs computer" }
                        </Link<Route>>
                <br/>
                <Link<Route>  to={Route::HowToPlayToot}>
                                { "How to Play TOOT-OTTO" }
                            </Link<Route>>
                            <Link<Route>  to={Route::TootOtto}>
                                { "play TOOT-OTTO vs human" }
                            </Link<Route>>
                            <Link<Route>  to={Route::TootOttoComputer}>
                                { "play TOOT-OTTO vs computer" }
                            </Link<Route>>
                <br/>
                <Link<Route>  to={Route::Scoreboard}>
                                { "Scoreboard" }
                            </Link<Route>>
                            <Link<Route>  to={Route::GameHistory}>
                                { "Game History" }
                            </Link<Route>>
            </nav>

        </div>
    }
}
