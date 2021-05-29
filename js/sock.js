function main() {
    let ws = new WebSocket("ws://localhost:8080");
    ws.binaryType = "arraybuffer";

    ws.onopen = () => {
        ws.send("doge");

        setTimeout(() => {
            ws.send("doge");
            ws.close();
        }, 3000);
    };
}

main();