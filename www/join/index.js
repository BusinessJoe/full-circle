console.log("Hello join")
document.addEventListener("DOMContentLoaded", function (event) {
    const chat = document.getElementById('chat');
    const text = document.getElementById('text');
    const send = document.getElementById('send');
    const uri = 'ws://' + '216.154.29.98:3001' + '/join';
    console.log(uri);
    const ws = new WebSocket(uri);
    function message(data) {
        const line = document.createElement('p');
        line.innerText = data;
        chat.appendChild(line);
    }
    ws.onopen = function () {
        chat.innerHTML = '<p><em>Connected!</em></p>';
    };
    ws.onmessage = function (msg) {
        message(msg.data);
    };
    ws.onclose = function () {
        chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
    };
    send.onclick = function () {
        const msg = text.value;
        ws.send(msg);
        text.value = '';
        message('<You>: ' + msg);
    };
});