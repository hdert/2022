use rand::seq::SliceRandom;
use std::vec;

use yew::prelude::*;

enum Msg {
    AddOne,
    SubtractOne,
    NewPrompt,
}

struct Model {
    value: i64,
    prompt: &'static str,
    unused_prompts: Vec<&'static str>,
}

// _______ 7 underscores per line

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            prompt: "Placeholder Prompt",
            unused_prompts: vec!["You dip your chips in sauce? I dip mine in _______.", "Frazer really likes _______.", "Gareth really likes _______.", "Chloe really likes _______.", "Ivan really likes _______.", "Schools all around the country have now banned _______.", "A brand new “position”, the _______.", "Introducing the new high school _______ club.", "The real reason why the substitute teacher left.", "Nine out of ten students agreed that there needs to be more _______ in the learning environment.", "What left this stain on my couch?", "Money can't buy me love, but it can buy me _______.", "Bought a huge 70-inch 4k TV, I can finally watch _______ now.", "I never leave the house without _______.", "In order to be hip with the kids, (card reader) is actively engaged in _______.", "Life would be better without _______.", "I'm not like other girls, I'm _______.", "What is in (card reader's) wardrobe?", "When my kitchen gets upgraded, what it really needs is _______.", "Define: Bruh Moment.", "1,2,3,4, what's behind that bloody door?", "New from Hot Wheels, the _______-mobile", "When I saw your mum, it reminded me of _______.", "I have varied tastes, I really enjoy eating _______.", "What is the green emergency exit man running from", "_______? That's not going to fit.", "Frosted Flakes; They're _______!", "Honey, did you really think it was a good idea to give the kids _______ for Christmas?", "I find your lack of _______ disturbing.", "It was all fun and games until _______.", "_______.com", "What do you stock up on in case of a zombie apocalypse?", "This one time, I stuck my _______ in my _______.", "The greatest rap battle in history! _______ vs. _______", "That's enough _______ for today.", "What's my specialty?", "I'm up shit creek without _______.", "Why can't I just be _______?", "Wow! Look! It's _______ on that _______. ", "_______, approved for under 18s.", "Bill Nye, The _______ Guy!", "You. Me. _______. Now.", "111, what's your emergency?", "This workplace has been 1 day without _______.", "Damn kids and their _______.", "On a scale from _______ to _______, how would you rate your pain?", "Instructions Unclear. _______ stuck in _______.", "I want _______ on my desk by 5, or you're fired!", "I couldn't find _______, so I had to use _______ instead.", "16 people. 39 days of _______. One Survivor.", "All classes today are cancelled due to _______.", "If you like _______, you'll love _______.", "But muuuum! I don't want _______ for dinner again!", "FOR SALE: _______. Only used once.", "Good news is, I'm _______. Bad news is, I'm _______.", "Home is where _______ is.", "Oh shit! I just got _______.", "The bathroom was out of toilet paper, so I used _______ on my ass instead.", "_______ is where my life started going wrong.", "<a href='https://ngtgyu.hdert.com/'>This is</a> (card reader's) favorite website _______.",],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::SubtractOne => {
                self.value -= 1;
                true
            }
            Msg::NewPrompt => {
                let chosen = self.unused_prompts.choose(&mut rand::thread_rng()).unwrap();
                self.prompt = chosen;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
            <header>
                <h1>{ "Card Generator" }</h1>
            </header>

            <div class="promptdisplay" id="cardDisplay">
                { self.prompt }
            </div>

            <button onclick={link.callback(|_| Msg::NewPrompt)}>{ "New Prompt" }</button>

            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={link.callback(|_| Msg::SubtractOne)}>{ "-1" }</button>
                <p>{ self.value }</p>
            </div>

            <footer class="footer">
                <object data="img/Copyleft.svg" type="image/svg+xml" style="height: 20px;"></object>
                <small>
                    // <!--I really want the copy left symbol-->
                { "big_man_gareth" }
                </small>
            </footer>
            </>
        }
    }
}

fn main() {
    // let prompts: Vec<&str> = vec!["You dip your chips in sauce? I dip mine in _______.", "Frazer really likes _______.", "Gareth really likes _______.", "Chloe really likes _______.", "Ivan really likes _______.", "Schools all around the country have now banned _______.", "A brand new “position”, the _______.", "Introducing the new high school _______ club.", "The real reason why the substitute teacher left.", "Nine out of ten students agreed that there needs to be more _______ in the learning environment.", "What left this stain on my couch?", "Money can't buy me love, but it can buy me _______.", "Bought a huge 70-inch 4k TV, I can finally watch _______ now.", "I never leave the house without _______.", "In order to be hip with the kids, (card reader) is actively engaged in _______.", "Life would be better without _______.", "I'm not like other girls, I'm _______.", "What is in (card reader's) wardrobe?", "When my kitchen gets upgraded, what it really needs is _______.", "Define: Bruh Moment.", "1,2,3,4, what's behind that bloody door?", "New from Hot Wheels, the _______-mobile", "When I saw your mum, it reminded me of _______.", "I have varied tastes, I really enjoy eating _______.", "What is the green emergency exit man running from", "_______? That's not going to fit.", "Frosted Flakes; They're _______!", "Honey, did you really think it was a good idea to give the kids _______ for Christmas?", "I find your lack of _______ disturbing.", "It was all fun and games until _______.", "_______.com", "What do you stock up on in case of a zombie apocalypse?", "This one time, I stuck my _______ in my _______.", "The greatest rap battle in history! _______ vs. _______", "That's enough _______ for today.", "What's my specialty?", "I'm up shit creek without _______.", "Why can't I just be _______?", "Wow! Look! It's _______ on that _______. ", "_______, approved for under 18s.", "Bill Nye, The _______ Guy!", "You. Me. _______. Now.", "111, what's your emergency?", "This workplace has been 1 day without _______.", "Damn kids and their _______.", "On a scale from _______ to _______, how would you rate your pain?", "Instructions Unclear. _______ stuck in _______.", "I want _______ on my desk by 5, or you're fired!", "I couldn't find _______, so I had to use _______ instead.", "16 people. 39 days of _______. One Survivor.", "All classes today are cancelled due to _______.", "If you like _______, you'll love _______.", "But muuuum! I don't want _______ for dinner again!", "FOR SALE: _______. Only used once.", "Good news is, I'm _______. Bad news is, I'm _______.", "Home is where _______ is.", "Oh shit! I just got _______.", "The bathroom was out of toilet paper, so I used _______ on my ass instead.", "_______ is where my life started going wrong.", "<a href='https://ngtgyu.hdert.com/'>This is</a> (card reader's) favorite website _______.",];

    yew::start_app::<Model>();
}
