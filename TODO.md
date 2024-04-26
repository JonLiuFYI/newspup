Now: implementing timer

* automate cargo-about licensing info (in CI?)
* clean up README

* figure out proper MinSec semantics. The fifteen-second steps should probably use their own struct instead of "abusing" MinSec
* refine timer UI
    * follow mockup: timer select
    * follow mockup: timer started
* probably just full reset the timer when app is relaunched: always return to Stopped
    * currently only resets if timer was running when app was closed
    * how to detect TimerState at the moment app is relaunched?

---

* code cleanup
    * player usize -> a newtype?
* choose scoring algorithm based on number of players: 1p, 2p, multi
    * are there i32 dragvalues?
    * dep injection or strategy pattern? Or just grab num_players at runtime?
    * So far, implemented the rules for 2p and 1p, but actually handling 2p might need a rework. While 3+p eliminates on Sunday, 2p applies a -10 pt penalty, which should be calculated in the model, not just calced on the spot in the score UI. For now, app is limited to 3p.
    * don't forget to allow selecting 1 and 2 players
* deploy

* code cleanup

## Gather licensing info before release
https://github.com/embarkstudios/cargo-about

    cargo about generate about.hbs -o licenses.html

## Scorecard sections
📰📷🌟⛶😿❎💰🏆
1. Articles
2. Photos
3. Centerpiece
4. Whitespace
5. Mood
6. Leftovers
7. Ads
8. Score
