Now: implementing timer

* make a Timer struct to hold timer data?
* time select screen
    * drag seconds in steps of 5 or 15 seconds
    * make it big and centred
    * make preset buttons set dragvalues
    * pressing start locks in selected min and sec, calculated into selected_duration
* refine timer UI
    * follow mockup: Stopped page shows duration select, the rest show styled timer
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