import { Application, createWindow, cleanup, requestQuit } from "./bindings"

export interface WindowAttributes {
  width: number,
  height: number
  title: string
}

export class Window {
  private _width: number
  private _height: number
  private _label: string

  constructor(label: string,  { width = 800, height = 600, title  = "Usumaki"}: Partial<WindowAttributes> = {}) {
    this._width = width;
    this._height = height;
    this._label = label;

    createWindow({width, height, label, title })
  }

  close() { }

  get width(): number {
    return this._width;
  }

  get height(): number {
    return this._height;
  }

  get label(): string {
    return this._label;
  }
}


// todo in future we dont want the user to allow doing this
export function runApp(entryFilePath: string) {
  let app = new Application();

  process.on("SIGINT", () => { });
  process.on("SIGTERM", () => { });

  console.log(entryFilePath);


  app.onInit(() => {
    const worker = new Worker(new URL("./main.ts", import.meta.url), {
      env: { ...process.env, entryPoint: entryFilePath }
    })
    worker.onerror = (e) => console.error("worker error:", e)
  })

  app.onWindowEvent(() => {
    // console.log("window event")
  })

  app.run()

  console.log("Reach here")
  cleanup();
}
