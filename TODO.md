How to organize code for calculating round scores?
Counting non-whitespace points is easy: just add/subtract parts of the ScoreColumn. How to check whitespace points? Where does that code go?

* show points from this round
    * implement whitespace points
    * refactor score sum and whitespace code into the right place
* show points and dollars in total
* live-indicate who's getting whitespace bonus/penalty

---

* score results page (so must have score calc logic)
    * dep injection or strategy pattern to choose scoring algorithm: 1p, 2p, multi
* automate cargo-about licensing info (in CI?)
* AGPL license headers
    * https://www.gnu.org/licenses/gpl-howto.html
* deploy

* code cleanup
* Sunday page declares winner

## Gather licensing info before release
https://github.com/embarkstudios/cargo-about

    cargo about generate about.hbs -o licenses.html

## (name, ScoreColumn) iteration
Manual way of iterating over a round's Vec as (name, ScoreColumn). Can I roll this into a trait to make it more ergonomic?

```
self.names
    .iter()
    .zip(self.scores[round].iter())
    .for_each(|(n, sc)| {
        todo!();
    });
```

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