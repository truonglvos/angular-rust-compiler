import { Injectable } from '@angular/core';
import * as i0 from '@angular/core';
export class ApiServices {
  static ɵfac = function ApiServices_Factory(__ngFactoryType__) {
    return new (__ngFactoryType__ || ApiServices)();
  };
  static ɵprov = /*@__PURE__*/ i0.ɵɵdefineInjectable({
    token: ApiServices,
    factory: ApiServices.ɵfac,
    providedIn: 'root',
  });
}
(() => {
  (typeof ngDevMode === 'undefined' || ngDevMode) &&
    i0.ɵsetClassMetadata(
      ApiServices,
      [
        {
          type: Injectable,
          args: [
            {
              providedIn: 'root',
            },
          ],
        },
      ],
      null,
      null,
    );
})();
