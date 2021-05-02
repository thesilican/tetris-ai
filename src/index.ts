import { Server } from "ws";
import { ITetrisAI, objIsAIRequest, RustyAI } from "./ai";

const ai: ITetrisAI = new RustyAI();

const server = new Server({ port: 8080 });
server.on("listening", () => console.log("Listening on port 8080"));
server.on("connection", (socket) => {
  console.log("New Socket connection");
  socket.on("message", async (data) => {
    let req: any;
    try {
      const str = data.toString();
      req = JSON.parse(str);
    } catch {
      console.error("Invalid JSON");
      return socket.send(JSON.stringify(null));
    }
    if (!objIsAIRequest(req)) {
      console.log("Invalid request");
      return socket.send(JSON.stringify(null));
    }
    const res = await ai.evaluate(req);
    socket.send(JSON.stringify(res));
  });
  socket.on("close", () => {
    console.log("Socket closed");
  });
});
