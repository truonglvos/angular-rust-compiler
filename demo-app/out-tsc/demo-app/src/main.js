import { bootstrapApplication } from "@angular/platform-browser";
import { appConfig } from "./app/app.config";
import { App } from "./app/app";
bootstrapApplication(App, appConfig).catch((err) => console.error(err));

console.log('%cAngular Rust compiler powered by Truonglv4', 'color: #00ff00; font-weight: bold;');
