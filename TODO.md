* choose scoring algorithm based on number of players: 1p, 2p, multi
    * dep injection or strategy pattern? Or just grab num_players at runtime?

---

* player usize -> a newtype?
* timer
* automate cargo-about licensing info (in CI?)
* link to Newspup source
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