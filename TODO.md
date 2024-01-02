situation now:
score model uses Round values as keys
more ergonomic for handling editintg scores (see page_scores.rs)
but implementation is ugly, since it uses HashMaps when I feel like a more appropriate option exists. Somehow map enum variants to struct fields?

* make a scoreboard
* player names