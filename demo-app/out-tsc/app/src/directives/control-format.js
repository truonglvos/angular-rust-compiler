import { Directive, HostListener, Input } from '@angular/core';
import * as i0 from '@angular/core';
import * as i1 from '@angular/forms';
export class ControlFormat {
  el;
  control;
  renderer2;
  characterPrevention = null;
  constructor(el, control, renderer2) {
    this.el = el;
    this.control = control;
    this.renderer2 = renderer2;
  }
  onInput(target) {
    /* set value on HTML */
    this.renderer2.setValue(this.el.nativeElement, '10');
    /* set value on model */
    this.control.control?.setValue('10');
  }
  onPaste(target) {
    this.renderer2.setValue(this.el.nativeElement, '10');
    this.control.control?.setValue('20');
  }
  static ɵfac = function ControlFormat_Factory(__ngFactoryType__) {
    return new (__ngFactoryType__ || ControlFormat)(
      i0.ɵɵdirectiveInject(i0.ElementRef),
      i0.ɵɵdirectiveInject(i1.NgControl),
      i0.ɵɵdirectiveInject(i0.Renderer2),
    );
  };
  static ɵdir = /*@__PURE__*/ i0.ɵɵdefineDirective({
    type: ControlFormat,
    selectors: [['', 'appControlFormat', '']],
    hostBindings: function ControlFormat_HostBindings(rf, ctx) {
      if (rf & 1) {
        i0.ɵɵlistener('input', function ControlFormat_input_HostBindingHandler($event) {
          return ctx.onInput($event.target);
        })('paste', function ControlFormat_paste_HostBindingHandler($event) {
          return ctx.onPaste($event.target);
        });
      }
    },
    inputs: { characterPrevention: 'characterPrevention' },
  });
}
(() => {
  (typeof ngDevMode === 'undefined' || ngDevMode) &&
    i0.ɵsetClassMetadata(
      ControlFormat,
      [
        {
          type: Directive,
          args: [
            {
              selector: '[appControlFormat]',
            },
          ],
        },
      ],
      () => [{ type: i0.ElementRef }, { type: i1.NgControl }, { type: i0.Renderer2 }],
      {
        characterPrevention: [
          {
            type: Input,
          },
        ],
        onInput: [
          {
            type: HostListener,
            args: ['input', ['$event.target']],
          },
        ],
        onPaste: [
          {
            type: HostListener,
            args: ['paste', ['$event.target']],
          },
        ],
      },
    );
})();
