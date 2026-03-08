// @bun
// ../../crates/usumaki_core/js/main.ts
console.log("worker started");
var entryPoint = process.env.entryPoint;
if (!entryPoint) {
  throw new Error("entryPoint not set");
}
try {
  await import(entryPoint);
} catch (e) {
  console.error("Error running entry point");
  console.error(e);
  process.exit(1);
}
