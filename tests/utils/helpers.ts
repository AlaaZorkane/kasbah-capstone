import { type DestinationStream, type Logger, pino } from "pino";
import { build } from "pino-pretty";

const BASE_OPTIONS = {
  colorize: true,
  colorizeObjects: true,
  sync: true,
} as const;

const prettyStream = build({
  ...BASE_OPTIONS,
  ignore: "hostname,pid,time",
});
const prettyStreamWithTimestamp = build({
  ...BASE_OPTIONS,
  ignore: "hostname,pid",
});

let fatalLoggerInstalled = false;
function ensureFatalLogger(logger: Logger<never>) {
  if (fatalLoggerInstalled) {
    return;
  }
  fatalLoggerInstalled = true;
  process.on("uncaughtException", (err) => {
    logger.fatal(err);
    process.exit(1);
  });
}

function createLoggerWithName(name: string, stream: DestinationStream) {
  return pino(
    {
      level: "debug",
      name,
    },
    stream,
  );
}

export function createLogger(name: string) {
  const logger = createLoggerWithName(name, prettyStream);
  ensureFatalLogger(logger);
  return logger;
}

export function createLoggerWithTimestamp(name: string) {
  const logger = createLoggerWithName(name, prettyStreamWithTimestamp);
  ensureFatalLogger(logger);
  return logger;
}

export function explorerUrl(tx: string, cluster = "devnet") {
  return `https://explorer.solana.com/tx/${tx}?cluster=${cluster}`;
}

export function explorerLocalUrl(tx: string) {
  return `https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http://127.0.0.1:8899`;
}

export function solFmLocalUrl(tx: string) {
  return `https://solana.fm/tx/${tx}?cluster=localnet-solana`;
}

export function solscanLocalUrl(tx: string) {
  return `https://solscan.io/tx/${tx}?cluster=custom&customUrl=http://127.0.0.1:8899`;
}
