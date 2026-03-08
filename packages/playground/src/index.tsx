import { Window  } from "usumaki"
import { render } from "usumaki/react"
import { App } from "./app"

const window = new Window("main", { title: "Somestupid shit" })

render(window, <App />)
