import * as i0 from "@angular/core";
import { NgFor } from "@angular/common";
import { ChangeDetectionStrategy, Component } from "@angular/core";
function NgForTest_div_2_Template(rf, ctx) {
	if (rf & 1) {
		i0.ɵɵelementStart(0, "div", 1);
		i0.ɵɵtext(1);
		i0.ɵɵelementEnd();
	}
	if (rf & 2) {
		const item_r1 = ctx.$implicit;
		i0.ɵɵadvance();
		i0.ɵɵtextInterpolate1("", item_r1, " 2");
	}
}
export class NgForTest {
	items = [
		"item 1",
		"item 2",
		"item 3"
	];
	static ɵfac = function NgForTest_Factory(t) {
		return new (t || NgForTest)();
	};
	static ɵcmp = /* @__PURE__ */ i0.ɵɵdefineComponent({
		type: NgForTest,
		selectors: [["app-ng-for"]],
		decls: 3,
		vars: 1,
		consts: [[
			"class",
			"ngfor-test",
			4,
			"ngFor",
			"ngForOf"
		], [1, "ngfor-test"]],
		template: function NgForTest_Template(rf, ctx) {
			if (rf & 1) {
				i0.ɵɵelementStart(0, "p");
				i0.ɵɵtext(1, "ng-for works!");
				i0.ɵɵelementEnd();
				i0.ɵɵtemplate(2, NgForTest_div_2_Template, 2, 1, "div", 0);
			}
			if (rf & 2) {
				i0.ɵɵadvance(2);
				i0.ɵɵproperty("ngForOf", ctx.items);
			}
		},
		standalone: true,
		styles: [""],
		changeDetection: 0,
		dependencies: [NgFor]
	});
}
