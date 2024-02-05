Score model has been reworked to use a struct and not a HashMap. Now:

* clean up code
* make method to grab &mut vec of ScoreColumns
* try refactoring Scoreboard to use tuple struct
    * can the Round enum act as indexing key? maybe try impl'ing IndexMut

---

* player names
* score results page (so must have score calc logic)

Scorecard sections: 📰📷🌟⛶😿❎💰🏆
1. Articles
2. Photos
3. Centerpiece
4. Whitespace
5. Mood
6. Leftovers
7. Ads
8. Score