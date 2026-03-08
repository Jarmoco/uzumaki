#!/usr/bin/env bun

import { $, fileURLToPath } from 'bun';
import { runApp } from './index';
import path from 'node:path';

const cmd = process.argv[2];

switch (cmd) {
  case 'run': {
    const entryPoint = process.argv[3];
    if (!entryPoint) {
      console.error('usage: usumaki run <entry-point.ts>');
      process.exit(1);
    }
    await run(entryPoint);
    break;
  }
  case 'build': {
    const entryPoint = process.argv[3];
    if (!entryPoint) {
      console.error('usage: usumaki build <entry-point.ts>');
      process.exit(1);
    }

    console.log('building...');
    await build(entryPoint);
    console.log('done');
    break;
  }
  default: {
    const entryPoint = process.argv[2];
    if (!entryPoint) {
      console.error('usage: usumaki <entry-point.ts>');
      process.exit(1);
    }
    await run(entryPoint);
  }
}

function resolveEntryPoint(entryPoint: string): string {
  return path.resolve(process.cwd(), entryPoint);
}

async function run(entryPoint: string) {
  const entryFile = resolveEntryPoint(entryPoint);
  if (!(await Bun.file(entryFile).exists())) {
    console.error(`entry point not found: ${entryPoint}`);
    process.exit(1);
  }

  runApp({ entryFilePath: entryFile });
}

function isWindows(): boolean {
  return process.platform === 'win32';
}

function normalizePathWindows(path: string): string {
  return path.replace(/\\/g, '/');
}

async function build(entryPoint: string) {
  const entryFile = resolveEntryPoint(entryPoint);

  if (!(await Bun.file(entryFile).exists())) {
    console.error(`entry point not found: ${entryPoint}`);
    process.exit(1);
  }
  // create a wrapper js function so that we can run the app

  const code = `
    import { runApp } from 'usumaki';
    runApp({ entryFilePath: './index' });
  `;

  await $`mkdir -p dist`;

  await $`echo '${code}' > dist/launch.js`;

  await Bun.build({
    entrypoints: [
      path.resolve(process.cwd(), 'dist/launch.js'),
      fileURLToPath(new URL('./main.ts', import.meta.url)),
      entryFile,
      /** main worker */
    ],
    naming: '[name].[ext]',
    external: ['usumaki'],
    splitting: true,
    target: 'bun',
    outdir: 'dist',
  });

  await Bun.build({
    entrypoints: ['dist/launch.js', 'dist/main.js', 'dist/index.js'],
    splitting: true,
    compile: true,
    naming: '[name].[ext]',
    target: 'bun',
  });

  // await Bun.build({
  //   entrypoints: ['dist/launch.js', 'dist/main'],
  //   compile: true,
  // });

  // await $`rm -rf dist`;
}
