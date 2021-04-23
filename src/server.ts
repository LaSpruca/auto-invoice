import sirv from "sirv";
import polka from "polka";
import compression from "compression";
import * as sapper from "@sapper/server";

const { PORT, NODE_ENV } = process.env;
const dev = NODE_ENV === "development";

const server = polka(); // You can also use Express
server.use(
  // @ts-ignore
  compression({ threshold: 0 }),
  sirv("static", { dev }),
  sapper.middleware()
);

server.listen(PORT, () => {});

export default server;
