import * as i0 from "@angular/core";
import { Pipe } from "@angular/core";
export class FullNamePipe {
	transform(name, surname) {
		return `${name} ${surname}`;
	}
	static ɵfac = function FullNamePipe_Factory(t) {
		return new (t || FullNamePipe)();
	};
	static ɵpipe = /* @__PURE__ */ i0.ɵɵdefinePipe({
		name: "fullName",
		type: FullNamePipe,
		pure: true,
		standalone: true
	});
}
