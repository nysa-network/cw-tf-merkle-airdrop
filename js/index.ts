#!/usr/bin/env node

import yargs from "yargs/yargs";
import { command } from "./cmd/generate-proof";

// @ts-ignore
process.server = true;

const argv = yargs(process.argv.slice(2))
  .commandDir("cmd", {
    extensions: ["js", "ts"],
  })
  .demandCommand()
  .help().argv;
