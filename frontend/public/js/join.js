const {drawCircle, initWebsocket} = require('./common.js');


async function run() {
    const room_id = location.pathname.split('/')[2];
    console.log(room_id);
    const uri = 'ws://' + location.hostname + ':3001' + '/join/' + room_id;
    initWebsocket(uri);
}

window.addEventListener("load", async function() {
    console.log("loaded");
    await run();
});
