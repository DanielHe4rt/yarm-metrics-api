# YARS - Yet Another Rhythm Statistics 


## Features

- [ ] Leaderboard per song [reference](https://clonehero.scorespy.online/leaderboards/95FD6F3E703C10437E882698004F3B01)
- [ ] Fetch a specific song score per user/score_id [reference](https://clonehero.scorespy.online/staticAssets/GeneratedSubmissions/2217413554242461101.png?t=1664089648)
- [ ] Most all time played songs [reference](https://clonehero.scorespy.online/api/v1/public/homeFeed)
- [ ] List of played songs
- [ ] Add Endpoints
  - [x] POST - /songs/metrics 
  - [x] GET - /songs/{song_id}
  - [ ] GET - /songs/last-submissions
  - [ ] GET - /leaderboard/
  - [ ] GET - /leaderboard/songs/all-time
  - [ ] GET - /leaderboard/players


### Leaderboard Per Song

* Rules:
  * Uma música pode ter apenas 1 estatistica de submissão por usuário
  * select song_id, user_id FROM song_submissions; // clustering order on score?
  * 