import { try_load_games, Schedule, Calendar } from "season_ticket_draft";
// import { dirname } from 'path';
// // import { fileURLToPath } from 'url';
// const fs = require('fs');
//
// const dirTree = require("directory-tree");
// co"./change_this_path"nst path = require("path");

// console.log(try_load_games);
// console.log(Schedule);
// // console.log(dirTree(__dirname));
// const games_path = path.resolve(__dirname, 'resources/games.json');
//
// console.log(games_path);

// const schedule = Schedule.new_2021_season();

// console.log(schedule);

const schedule_elem = document.getElementById("game-schedule")

const calendar = Calendar.try_new(10, 2021, 6, 2022);

const renderLoop = () => {
    schedule_elem.innerHTML = `<div>${calendar.render()}</div>`

    // requestAnimationFrame(renderLoop);
};

// function gameSelect() {
//     const game = Game
// }

requestAnimationFrame(renderLoop);
