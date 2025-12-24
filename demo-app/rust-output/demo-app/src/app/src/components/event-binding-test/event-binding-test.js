import * as i0 from "@angular/core";
import { Component } from "@angular/core";
export class EventBindingTest {
	clickCount = 0;
	onClick() {
		this.clickCount++;
	}
	static ɵfac = function EventBindingTest_Factory(t) {
		return new (t || EventBindingTest)();
	};
	static ɵcmp = /* @__PURE__ */ i0.ɵɵdefineComponent({
		type: EventBindingTest,
		selectors: [["app-event-binding-test"]],
		decls: 6,
		vars: 1,
		consts: [[3, "click"]],
		template: function EventBindingTest_Template(rf, ctx) {
			if (rf & 1) {
				i0.ɵɵelementStart(0, "p");
				i0.ɵɵtext(1, "event-binding-test works!");
				i0.ɵɵelementEnd();
				i0.ɵɵelementStart(2, "button", 0);
				i0.ɵɵlistener("click", function EventBindingTest_buttonclick_0_listener() {
					return ctx.onClick();
				});
				i0.ɵɵtext(3, "Click me");
				i0.ɵɵelementEnd();
				i0.ɵɵelementStart(4, "p");
				i0.ɵɵtext(5);
				i0.ɵɵelementEnd();
			}
			if (rf & 2) {
				i0.ɵɵadvance(5);
				i0.ɵɵtextInterpolate1("Clicked ", ctx.clickCount, " times");
			}
		},
		standalone: true,
		styles: [""]
	});
}
