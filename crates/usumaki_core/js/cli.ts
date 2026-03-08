#!/usr/bin/env bun

import { runApp } from "./index"
import path from "node:path"

const entryPoint = process.argv[2]
if(!entryPoint) {
  console.error("usage: usumaki <entry-point.ts>")
  process.exit(1)
}

const entryFile = path.resolve(process.cwd(), entryPoint)

if (!await Bun.file(entryFile).exists()) {
  console.error(`entry point not found: ${entryPoint}`)
  process.exit(1)
}

runApp(entryFile)

// runApp(() => {

// });
