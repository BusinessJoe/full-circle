console.log("im a peer")

// Websocket stuff
let id = new URLSearchParams(window.location.search).get("id");
console.log(id);
const uri = 'ws://' + 'localhost:3030' + '/chat';
const ws = new WebSocket(uri);


async function makeCall() {
    const configuration = { 'iceServers': [{ 'urls': 'stun:stun.l.google.com:19302' }] }
    const peerConnection = new RTCPeerConnection(configuration);
    const dataChannel = peerConnection.createDataChannel();
    ws.onmessage = async message => {
        if (message.answer) {
            const remoteDesc = new RTCSessionDescription(message.answer);
            await peerConnection.setRemoteDescription(remoteDesc);
        }
    };
    const offer = await peerConnection.createOffer();
    await peerConnection.setLocalDescription(offer);
    ws.send({ 'offer': offer });
}

ws.onload = makeCall;