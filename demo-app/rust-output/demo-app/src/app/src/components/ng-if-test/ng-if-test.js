import * as i0 from "@angular/core";
import { Component } from "@angular/core";
function NgIfTest_p_0_Template(rf, ctx) {
	if (rf & 1) {
		i0.ɵɵelementStart(0, "p");
		i0.ɵɵtext(1, "ng-if-test works!");
		i0.ɵɵelementEnd();
	}
}
function NgIfTest_p_1_Template(rf, ctx) {
	if (rf & 1) {
		i0.ɵɵelementStart(0, "p");
		i0.ɵɵtext(1, "dhơi mêt roi");
		i0.ɵɵelementEnd();
	}
}
export class NgIfTest {
	isShow = 1;
	static ɵfac = function NgIfTest_Factory(t) {
		return new (t || NgIfTest)();
	};
	static ɵcmp = /* @__PURE__ */ i0.ɵɵdefineComponent({
		type: NgIfTest,
		selectors: [["app-ng-if-test"]],
		decls: 4,
		vars: 2,
		consts: [[4, "ngIf"]],
		template: function NgIfTest_Template(rf, ctx) {
			if (rf & 1) {
				i0.ɵɵtemplate(0, NgIfTest_p_0_Template, 2, 0, "p", 0);
				i0.ɵɵtemplate(1, NgIfTest_p_1_Template, 2, 0, "p", 0);
				i0.ɵɵelementStart(2, "button");
				i0.ɵɵtext(3, "Click me");
				i0.ɵɵelementEnd();
			}
			if (rf & 2) {
				i0.ɵɵproperty("ngIf", ctx.isShow);
				i0.ɵɵadvance();
				i0.ɵɵproperty("ngIf", !ctx.isShow);
			}
		},
		standalone: true,
		styles: [""]
	});
}
