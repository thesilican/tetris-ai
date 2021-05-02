import chalk from "chalk";
import { spawn } from "child_process";
import { ChildProcessWithoutNullStreams } from "node:child_process";
import { ReadLine } from "node:readline";
import path from "path";
import readline from "readline";
import { AIRequest, AIResponse, ITetrisAI, objIsAIResponse } from "./index";

export abstract class CLIAI implements ITetrisAI {
  protected name: string;
  protected child: ChildProcessWithoutNullStreams;
  protected rl: ReadLine;

  constructor(command: string, args: string[]) {
    this.name = path.basename(command);
    this.child = spawn(command, args, { stdio: "pipe" });
    this.rl = readline.createInterface({
      input: this.child.stdout,
      output: this.child.stdin,
      terminal: false,
    });
    this.child.stderr.on("data", (data: Buffer) => {
      console.error(chalk.cyan(data.toString().trim()));
    });
    this.child.on("exit", (code) => {
      console.error(this.name + " exited prematurely with code: ", code);
      process.exit(1);
    });
    process.on("exit", () => {
      this.child.kill("SIGTERM");
    });
  }

  async evaluate(req: AIRequest): Promise<AIResponse | null> {
    const out = JSON.stringify(req);
    const response = await new Promise<string>((res) =>
      this.rl.question(out + "\n", res)
    );
    const res = JSON.parse(response);
    if (!objIsAIResponse(res)) {
      return null;
    }
    return res;
  }
}
