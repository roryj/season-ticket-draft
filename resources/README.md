# Getting the kraken team data

``` sh
https://statsapi.web.nhl.com/api/v1/franchises

```

# Getting the game data

``` sh
curl "https://statsapi.web.nhl.com/api/v1/schedule?teamId=55&season=20212022&gameType=R" > src/resources/unprocessed.json
```
