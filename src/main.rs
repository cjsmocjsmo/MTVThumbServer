use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos_router::{components::*, path};

mod moviespage;
use crate::moviespage::MovCatListPage;

mod tvshowspage;
use crate::tvshowspage::TVShowsListPage;

mod searchpage;
use crate::searchpage::SearchPage;

mod movapages;
use crate::movapages::ActionPage;
use crate::movapages::ArnoldPage;

mod movbpages;
use crate::movbpages::BruceLeePage;
use crate::movbpages::BruceWillisPage;
use crate::movbpages::BuzzPage;

mod movcpages;
use crate::movcpages::CartoonsPage;
use crate::movcpages::CharlieBrownPage;
use crate::movcpages::ChuckNorrisPage;
use crate::movcpages::ComedyPage;

mod movdpages;
use crate::movdpages::DramaPage;
use crate::movdpages::DocumentaryPage;

mod movfpages;
use crate::movfpages::FantasyPage;

mod movgpages;
use crate::movgpages::GhostBusterPage;
use crate::movgpages::GodzillaPage;

mod movhpages;
use crate::movhpages::HarrisonFordPage;
use crate::movhpages::HarryPotterPage;
use crate::movhpages::HellBoyPage;

mod movipages;
use crate::movipages::IndianaJonesPage;

mod movjpages;
use crate::movjpages::JamesBondPage;
use crate::movjpages::JohnWaynePage;
use crate::movjpages::JohnWickPage;
use crate::movjpages::JurassicParkPage;

mod movkpages;
use crate::movkpages::KevinCostnerPage;
use crate::movkpages::KingsManPage;

mod movlpages;
use crate::movlpages::LegoPage;

mod movmpages;
use crate::movmpages::MenInBlackPage;
use crate::movmpages::MinionsPage;
use crate::movmpages::MiscPage;

mod movnpages;
use crate::movnpages::NicolasCagePage;

mod movopages;
use crate::movopages::OldiesPage;

mod movppages;
use crate::movppages::PandasPage;
use crate::movppages::PiratesPage;

mod movrpages;
use crate::movrpages::RiddickPage;

mod movspages;
use crate::movspages::SciFiPage;
use crate::movspages::StalonePage;
use crate::movspages::StarTrekPage;
use crate::movspages::StarWarsPage;
use crate::movspages::SuperHerosPage;

mod movtpages;
use crate::movtpages::TinkerBellPage;
use crate::movtpages::TomCruisePage;
use crate::movtpages::TransformersPage;
use crate::movtpages::TremorsPage;
use crate::movtpages::TheRockPage;

mod movvpages;
use crate::movvpages::VanDamPage;

mod movxpages;
use crate::movxpages::XMenPage;

fn main() {
	console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <NavBar />
            <Header />
            <main>
                <Routes fallback=|| "Not Found.">
                    <Route path=path!("/") view=MovCatListPage />
                    <Route path=path!("/tvshows") view=TVShowsListPage />
                    <Route path=path!("/search") view=SearchPage />
                    <Route path=path!("/action") view=ActionPage />
                    <Route path=path!("/arnold") view=ArnoldPage />
                    <Route path=path!("/brucelee") view=BruceLeePage />
                    <Route path=path!("/brucewillis") view=BruceWillisPage />
                    <Route path=path!("/buzz") view=BuzzPage />
                    <Route path=path!("/cartoons") view=CartoonsPage />
                    <Route path=path!("/charliebrown") view=CharlieBrownPage />
                    <Route path=path!("/chucknorris") view=ChuckNorrisPage />
                    <Route path=path!("/comedy") view=ComedyPage />
                    <Route path=path!("/documentary") view=DocumentaryPage />
                    <Route path=path!("/drama") view=DramaPage />
                    <Route path=path!("/fantasy") view=FantasyPage />
                    <Route path=path!("/ghostbuster") view=GhostBusterPage />
                    <Route path=path!("/godzilla") view=GodzillaPage />
                    <Route path=path!("/harrisonford") view=HarrisonFordPage />
                    <Route path=path!("/harrypotter") view=HarryPotterPage />
                    <Route path=path!("/hellboy") view=HellBoyPage />
                    <Route path=path!("/indianajones") view=IndianaJonesPage />
                    <Route path=path!("/jamesbond") view=JamesBondPage />
                    <Route path=path!("/johnwayne") view=JohnWaynePage />
                    <Route path=path!("/johnwick") view=JohnWickPage />
                    <Route path=path!("/jurassicpark") view=JurassicParkPage />
                    <Route path=path!("/kevincostner") view=KevinCostnerPage />
                    <Route path=path!("/kingsman") view=KingsManPage />
                    <Route path=path!("/lego") view=LegoPage />
                    <Route path=path!("/meninblack") view=MenInBlackPage />
                    <Route path=path!("/minions") view=MinionsPage />
                    <Route path=path!("/misc") view=MiscPage />
                    <Route path=path!("/nicolascage") view=NicolasCagePage />
                    <Route path=path!("/oldies") view=OldiesPage />
                    <Route path=path!("/pandas") view=PandasPage />
                    <Route path=path!("/pirates") view=PiratesPage />
                    <Route path=path!("/riddick") view=RiddickPage />
                    <Route path=path!("/scifi") view=SciFiPage />
                    <Route path=path!("/stalone") view=StalonePage />
                    <Route path=path!("/startrek") view=StarTrekPage />
                    <Route path=path!("/starwars") view=StarWarsPage />
                    <Route path=path!("/superheros") view=SuperHerosPage />
                    <Route path=path!("/tinkerbell") view=TinkerBellPage />
                    <Route path=path!("/tomcruise") view=TomCruisePage />
                    <Route path=path!("/transformers") view=TransformersPage />
                    <Route path=path!("/tremors") view=TremorsPage />
                    <Route path=path!("/therock") view=TheRockPage />
                    <Route path=path!("/vandam") view=VanDamPage />
                    <Route path=path!("/xmen") view=XMenPage />

                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <h1 class="header">"MTV"</h1>
        </header>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <a href="/" class="navItem">"Movies"</a>
            <a href="/tvshows" class="navItem">"TV Shows"</a>
            <a href="/search" class="navItem">"Search"</a>
        </nav>
    }
}