import { Pipe } from '@angular/core';
import * as i0 from '@angular/core';
export class FullNamePipe {
  transform(name, surname) {
    return `${name} ${surname}`;
  }
  static ɵfac = function FullNamePipe_Factory(__ngFactoryType__) {
    return new (__ngFactoryType__ || FullNamePipe)();
  };
  static ɵpipe = /*@__PURE__*/ i0.ɵɵdefinePipe({
    name: 'fullName',
    type: FullNamePipe,
    pure: true,
  });
}
(() => {
  (typeof ngDevMode === 'undefined' || ngDevMode) &&
    i0.ɵsetClassMetadata(
      FullNamePipe,
      [
        {
          type: Pipe,
          args: [
            {
              name: 'fullName',
            },
          ],
        },
      ],
      null,
      null,
    );
})();
