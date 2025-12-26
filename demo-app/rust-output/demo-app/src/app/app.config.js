import { provideBrowserGlobalErrorListeners, provideZonelessChangeDetection } from '@angular/core';
import { provideRouter } from '@angular/router';
import { routes } from './app.routes';
export const appConfig = { providers: [
	provideBrowserGlobalErrorListeners(),
	provideZonelessChangeDetection(),
	provideRouter(routes)
] };
