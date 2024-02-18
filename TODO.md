Score inputs now implemented. Now need to do score calc and show scoreboard at end of round

* what to show on score subpage?
    * this round and total
    * just total
* live-indicate who's getting whitespace bonus/penalty

---

* score results page (so must have score calc logic)
    * dep injection or strategy pattern to choose scoring algorithm: 1p, 2p, multi
* licensing info (ideally automated)
* deploy

* code cleanup
* Sunday page declares winner

Manual way of iterating over a round's Vec as (name, ScoreColumn). Can I roll this into a trait to make it more ergonomic?

```
self.names
    .iter()
    .zip(self.scores[round].iter())
    .for_each(|(n, sc)| {
        todo!();
    });
```

Scorecard sections: 📰📷🌟⛶😿❎💰🏆
1. Articles
2. Photos
3. Centerpiece
4. Whitespace
5. Mood
6. Leftovers
7. Ads
8. Score