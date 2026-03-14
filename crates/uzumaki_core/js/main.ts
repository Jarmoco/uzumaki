import { dispatchEvent } from './react/reconciler';
import { listenAppEvents } from './bindings';
import { AppEventKind } from './bindings';

console.log('worker started');
const entryPoint = process.env.entryPoint;

if (!entryPoint) {
  throw new Error('entryPoint not set');
}

listenAppEvents((err, event) => {
  if (err) {
    console.error('DOM event error:', err);
    return;
  }
  if (event.kind === AppEventKind.DomEvent && event.domEvent) {
    dispatchEvent(event.domEvent.nodeId, event.domEvent.eventType);
  } else if (event.kind === AppEventKind.HotReload) {
    console.log('hot reload');
  }
});

try {
  await import(entryPoint);
} catch (e) {
  console.error('Error running entry point');
  console.error(e);
  process.exit(1);
}
