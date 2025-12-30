import { Component, Input, Output, EventEmitter } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterLink } from '@angular/router';
import * as i0 from '@angular/core';
import * as i1 from '@angular/common';
const _c0 = () => ['/home'];
const _c1 = () => ({ ref: 'component67' });
const _forTrack0 = ($index, $item) => $item.id;
function Benchmark67Component_For_10_Template(rf, ctx) {
  if (rf & 1) {
    const _r1 = i0.ɵɵgetCurrentView();
    i0.ɵɵelementStart(0, 'li', 13);
    i0.ɵɵlistener('click', function Benchmark67Component_For_10_Template_li_click_0_listener() {
      const item_r2 = i0.ɵɵrestoreView(_r1).$implicit;
      const ctx_r2 = i0.ɵɵnextContext();
      return i0.ɵɵresetView(ctx_r2.select(item_r2));
    });
    i0.ɵɵelementStart(1, 'a', 14);
    i0.ɵɵtext(2);
    i0.ɵɵelementEnd()();
  }
  if (rf & 2) {
    const item_r2 = ctx.$implicit;
    const ctx_r2 = i0.ɵɵnextContext();
    i0.ɵɵclassProp('selected', item_r2.id === ctx_r2.selectedId);
    i0.ɵɵadvance();
    i0.ɵɵproperty('href', item_r2.url, i0.ɵɵsanitizeUrl);
    i0.ɵɵattribute('data-id', item_r2.id);
    i0.ɵɵadvance();
    i0.ɵɵtextInterpolate1(' ', item_r2.name, ' ');
  }
}
function Benchmark67Component_Conditional_12_Template(rf, ctx) {
  if (rf & 1) {
    i0.ɵɵelementStart(0, 'div', 15)(1, 'span');
    i0.ɵɵtext(2, 'Loading...');
    i0.ɵɵelementEnd()();
  }
  if (rf & 2) {
    const ctx_r2 = i0.ɵɵnextContext();
    i0.ɵɵstyleProp('width', ctx_r2.spinnerSize, 'px');
  }
}
function Benchmark67Component_Conditional_13_Template(rf, ctx) {
  if (rf & 1) {
    const _r4 = i0.ɵɵgetCurrentView();
    i0.ɵɵelementStart(0, 'div', 16)(1, 'p');
    i0.ɵɵtext(2);
    i0.ɵɵelementEnd();
    i0.ɵɵelementStart(3, 'button', 13);
    i0.ɵɵlistener(
      'click',
      function Benchmark67Component_Conditional_13_Template_button_click_3_listener() {
        i0.ɵɵrestoreView(_r4);
        const ctx_r2 = i0.ɵɵnextContext();
        return i0.ɵɵresetView(ctx_r2.retry());
      },
    );
    i0.ɵɵtext(4, 'Retry');
    i0.ɵɵelementEnd()();
  }
  if (rf & 2) {
    const ctx_r2 = i0.ɵɵnextContext();
    i0.ɵɵclassProp('critical', ctx_r2.isCritical);
    i0.ɵɵadvance(2);
    i0.ɵɵtextInterpolate(ctx_r2.errorMessage);
  }
}
function Benchmark67Component_Conditional_14_For_2_Template(rf, ctx) {
  if (rf & 1) {
    const _r5 = i0.ɵɵgetCurrentView();
    i0.ɵɵelementStart(0, 'article', 18);
    i0.ɵɵlistener(
      'mouseenter',
      function Benchmark67Component_Conditional_14_For_2_Template_article_mouseenter_0_listener() {
        const row_r6 = i0.ɵɵrestoreView(_r5).$implicit;
        const ctx_r2 = i0.ɵɵnextContext(2);
        return i0.ɵɵresetView(ctx_r2.onHover(row_r6));
      },
    )(
      'mouseleave',
      function Benchmark67Component_Conditional_14_For_2_Template_article_mouseleave_0_listener() {
        const row_r6 = i0.ɵɵrestoreView(_r5).$implicit;
        const ctx_r2 = i0.ɵɵnextContext(2);
        return i0.ɵɵresetView(ctx_r2.onLeave(row_r6));
      },
    );
    i0.ɵɵelementStart(1, 'div', 19);
    i0.ɵɵelement(2, 'img', 20);
    i0.ɵɵelementStart(3, 'h3');
    i0.ɵɵtext(4);
    i0.ɵɵelementEnd()();
    i0.ɵɵelementStart(5, 'div', 21)(6, 'p');
    i0.ɵɵtext(7);
    i0.ɵɵelementEnd();
    i0.ɵɵelementStart(8, 'span', 22);
    i0.ɵɵtext(9);
    i0.ɵɵpipe(10, 'date');
    i0.ɵɵelementEnd()();
    i0.ɵɵelementStart(11, 'div', 23)(12, 'button', 24);
    i0.ɵɵlistener(
      'click',
      function Benchmark67Component_Conditional_14_For_2_Template_button_click_12_listener() {
        const row_r6 = i0.ɵɵrestoreView(_r5).$implicit;
        const ctx_r2 = i0.ɵɵnextContext(2);
        return i0.ɵɵresetView(ctx_r2.edit(row_r6));
      },
    );
    i0.ɵɵtext(13, 'Edit');
    i0.ɵɵelementEnd();
    i0.ɵɵelementStart(14, 'button', 13);
    i0.ɵɵlistener(
      'click',
      function Benchmark67Component_Conditional_14_For_2_Template_button_click_14_listener() {
        const row_r6 = i0.ɵɵrestoreView(_r5).$implicit;
        const ctx_r2 = i0.ɵɵnextContext(2);
        return i0.ɵɵresetView(ctx_r2.delete(row_r6));
      },
    );
    i0.ɵɵtext(15, 'Delete');
    i0.ɵɵelementEnd()()();
  }
  if (rf & 2) {
    const row_r6 = ctx.$implicit;
    const ɵ$index_42_r7 = ctx.$index;
    const ɵ$count_42_r8 = ctx.$count;
    i0.ɵɵclassProp('first', ɵ$index_42_r7 === 0)('last', ɵ$index_42_r7 === ɵ$count_42_r8 - 1)(
      'even',
      ɵ$index_42_r7 % 2 === 0,
    );
    i0.ɵɵattribute('data-index', ɵ$index_42_r7);
    i0.ɵɵadvance(2);
    i0.ɵɵproperty('src', row_r6.image, i0.ɵɵsanitizeUrl)('alt', row_r6.title);
    i0.ɵɵadvance(2);
    i0.ɵɵtextInterpolate(row_r6.title);
    i0.ɵɵadvance(3);
    i0.ɵɵtextInterpolate(row_r6.description);
    i0.ɵɵadvance(2);
    i0.ɵɵtextInterpolate(i0.ɵɵpipeBind2(10, 15, row_r6.date, 'short'));
    i0.ɵɵadvance(3);
    i0.ɵɵproperty('disabled', row_r6.locked);
    i0.ɵɵadvance(2);
    i0.ɵɵclassProp('danger', true);
  }
}
function Benchmark67Component_Conditional_14_Template(rf, ctx) {
  if (rf & 1) {
    i0.ɵɵelementStart(0, 'section', 8);
    i0.ɵɵrepeaterCreate(
      1,
      Benchmark67Component_Conditional_14_For_2_Template,
      16,
      18,
      'article',
      17,
      _forTrack0,
    );
    i0.ɵɵelementEnd();
  }
  if (rf & 2) {
    const ctx_r2 = i0.ɵɵnextContext();
    i0.ɵɵadvance();
    i0.ɵɵrepeater(ctx_r2.data);
  }
}
function Benchmark67Component_div_16_Template(rf, ctx) {
  if (rf & 1) {
    i0.ɵɵelementStart(0, 'div', 25)(1, 'h4');
    i0.ɵɵtext(2);
    i0.ɵɵelementEnd();
    i0.ɵɵelement(3, 'div', 26);
    i0.ɵɵelementEnd();
  }
  if (rf & 2) {
    const widget_r9 = ctx.$implicit;
    i0.ɵɵadvance(2);
    i0.ɵɵtextInterpolate(widget_r9.title);
    i0.ɵɵadvance();
    i0.ɵɵproperty('innerHTML', widget_r9.content, i0.ɵɵsanitizeHtml);
  }
}
export class Benchmark67Component {
  title = 'Benchmark Component 67';
  subtitle = 'Performance Testing';
  action = new EventEmitter();
  isActive = false;
  isLoading = false;
  hasError = false;
  isCritical = false;
  errorMessage = '';
  selectedId = 0;
  spinnerSize = 50;
  footerColor = '#333';
  currentYear = new Date().getFullYear();
  items = Array.from({ length: 10 }, (_, i) => ({
    id: i,
    name: `Item ${i}`,
    url: `/item/${i}`,
  }));
  data = Array.from({ length: 5 }, (_, i) => ({
    id: i,
    title: `Row ${i}`,
    description: `Description for row ${i}`,
    image: `https://picsum.photos/100/100?random=${i}`,
    date: new Date(),
    locked: i % 3 === 0,
  }));
  widgets = Array.from({ length: 3 }, (_, i) => ({
    title: `Widget ${i}`,
    content: `<p>Widget content ${i}</p>`,
  }));
  handleClick(event) {
    this.isActive = !this.isActive;
  }
  select(item) {
    this.selectedId = item.id;
  }
  retry() {
    this.hasError = false;
    this.isLoading = true;
  }
  onHover(row) {}
  onLeave(row) {}
  edit(row) {}
  delete(row) {}
  static ɵfac = function Benchmark67Component_Factory(__ngFactoryType__) {
    return new (__ngFactoryType__ || Benchmark67Component)();
  };
  static ɵcmp = /*@__PURE__*/ i0.ɵɵdefineComponent({
    type: Benchmark67Component,
    selectors: [['app-benchmark-67']],
    inputs: { title: 'title', subtitle: 'subtitle' },
    outputs: { action: 'action' },
    decls: 22,
    vars: 15,
    consts: [
      [1, 'benchmark-component-67'],
      [1, 'header', 3, 'click'],
      [1, 'subtitle'],
      [1, 'navigation'],
      [3, 'selected'],
      [1, 'content'],
      [1, 'spinner', 3, 'width'],
      [1, 'error', 3, 'critical'],
      [1, 'data-section'],
      [1, 'sidebar'],
      ['class', 'widget', 4, 'ngFor', 'ngForOf'],
      [1, 'footer'],
      [3, 'routerLink', 'queryParams'],
      [3, 'click'],
      [3, 'href'],
      [1, 'spinner'],
      [1, 'error'],
      [1, 'data-row', 3, 'first', 'last', 'even'],
      [1, 'data-row', 3, 'mouseenter', 'mouseleave'],
      [1, 'row-header'],
      [3, 'src', 'alt'],
      [1, 'row-body'],
      [1, 'meta'],
      [1, 'row-actions'],
      [3, 'click', 'disabled'],
      [1, 'widget'],
      [3, 'innerHTML'],
    ],
    template: function Benchmark67Component_Template(rf, ctx) {
      if (rf & 1) {
        i0.ɵɵelementStart(0, 'div', 0)(1, 'header', 1);
        i0.ɵɵlistener(
          'click',
          function Benchmark67Component_Template_header_click_1_listener($event) {
            return ctx.handleClick($event);
          },
        );
        i0.ɵɵelementStart(2, 'h1');
        i0.ɵɵtext(3);
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(4, 'span', 2);
        i0.ɵɵtext(5);
        i0.ɵɵpipe(6, 'uppercase');
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(7, 'nav', 3)(8, 'ul');
        i0.ɵɵrepeaterCreate(9, Benchmark67Component_For_10_Template, 3, 5, 'li', 4, _forTrack0);
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(11, 'main', 5);
        i0.ɵɵconditionalCreate(12, Benchmark67Component_Conditional_12_Template, 3, 2, 'div', 6)(
          13,
          Benchmark67Component_Conditional_13_Template,
          5,
          3,
          'div',
          7,
        )(14, Benchmark67Component_Conditional_14_Template, 3, 0, 'section', 8);
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(15, 'aside', 9);
        i0.ɵɵtemplate(16, Benchmark67Component_div_16_Template, 4, 2, 'div', 10);
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(17, 'footer', 11)(18, 'p');
        i0.ɵɵtext(19);
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(20, 'a', 12);
        i0.ɵɵtext(21, 'Home');
        i0.ɵɵelementEnd()()();
      }
      if (rf & 2) {
        i0.ɵɵadvance();
        i0.ɵɵclassProp('active', ctx.isActive);
        i0.ɵɵadvance(2);
        i0.ɵɵtextInterpolate(ctx.title);
        i0.ɵɵadvance(2);
        i0.ɵɵtextInterpolate(i0.ɵɵpipeBind1(6, 11, ctx.subtitle));
        i0.ɵɵadvance(4);
        i0.ɵɵrepeater(ctx.items);
        i0.ɵɵadvance(3);
        i0.ɵɵconditional(ctx.isLoading ? 12 : ctx.hasError ? 13 : 14);
        i0.ɵɵadvance(4);
        i0.ɵɵproperty('ngForOf', ctx.widgets);
        i0.ɵɵadvance();
        i0.ɵɵstyleProp('background-color', ctx.footerColor);
        i0.ɵɵadvance(2);
        i0.ɵɵtextInterpolate1('\u00A9 ', ctx.currentYear, ' - Component 67');
        i0.ɵɵadvance();
        i0.ɵɵproperty('routerLink', i0.ɵɵpureFunction0(13, _c0))(
          'queryParams',
          i0.ɵɵpureFunction0(14, _c1),
        );
      }
    },
    dependencies: [CommonModule, i1.NgForOf, RouterLink, i1.UpperCasePipe, i1.DatePipe],
    encapsulation: 2,
  });
}
(() => {
  (typeof ngDevMode === 'undefined' || ngDevMode) &&
    i0.ɵsetClassMetadata(
      Benchmark67Component,
      [
        {
          type: Component,
          args: [
            {
              selector: 'app-benchmark-67',
              standalone: true,
              imports: [CommonModule, RouterLink],
              template:
                '<div class="benchmark-component-67">\n  <header class="header" [class.active]="isActive" (click)="handleClick($event)">\n    <h1>{{ title }}</h1>\n    <span class="subtitle">{{ subtitle | uppercase }}</span>\n  </header>\n\n  <nav class="navigation">\n    <ul>\n      @for (item of items; track item.id) {\n        <li [class.selected]="item.id === selectedId" (click)="select(item)">\n          <a [href]="item.url" [attr.data-id]="item.id">\n            {{ item.name }}\n          </a>\n        </li>\n      }\n    </ul>\n  </nav>\n\n  <main class="content">\n    @if (isLoading) {\n      <div class="spinner" [style.width.px]="spinnerSize">\n        <span>Loading...</span>\n      </div>\n    } @else if (hasError) {\n      <div class="error" [class.critical]="isCritical">\n        <p>{{ errorMessage }}</p>\n        <button (click)="retry()">Retry</button>\n      </div>\n    } @else {\n      <section class="data-section">\n        @for (row of data; track row.id; let i = $index, first = $first, last = $last) {\n          <article\n            class="data-row"\n            [class.first]="first"\n            [class.last]="last"\n            [class.even]="i % 2 === 0"\n            [attr.data-index]="i"\n            (mouseenter)="onHover(row)"\n            (mouseleave)="onLeave(row)"\n          >\n            <div class="row-header">\n              <img [src]="row.image" [alt]="row.title" />\n              <h3>{{ row.title }}</h3>\n            </div>\n            <div class="row-body">\n              <p>{{ row.description }}</p>\n              <span class="meta">{{ row.date | date: \'short\' }}</span>\n            </div>\n            <div class="row-actions">\n              <button [disabled]="row.locked" (click)="edit(row)">Edit</button>\n              <button [class.danger]="true" (click)="delete(row)">Delete</button>\n            </div>\n          </article>\n        }\n      </section>\n    }\n  </main>\n\n  <aside class="sidebar">\n    <div class="widget" *ngFor="let widget of widgets">\n      <h4>{{ widget.title }}</h4>\n      <div [innerHTML]="widget.content"></div>\n    </div>\n  </aside>\n\n  <footer class="footer" [style.backgroundColor]="footerColor">\n    <p>&copy; {{ currentYear }} - Component 67</p>\n    <a [routerLink]="[\'/home\']" [queryParams]="{ ref: \'component67\' }">Home</a>\n  </footer>\n</div>\n',
            },
          ],
        },
      ],
      null,
      {
        title: [
          {
            type: Input,
          },
        ],
        subtitle: [
          {
            type: Input,
          },
        ],
        action: [
          {
            type: Output,
          },
        ],
      },
    );
})();
(() => {
  (typeof ngDevMode === 'undefined' || ngDevMode) &&
    i0.ɵsetClassDebugInfo(Benchmark67Component, {
      className: 'Benchmark67Component',
      filePath: 'src/app/benchmark/benchmark-67.component.ts',
      lineNumber: 11,
    });
})();
