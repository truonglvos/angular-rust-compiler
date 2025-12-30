import { Component, Input, Output, EventEmitter } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterLink } from '@angular/router';
import * as i1 from '@angular/common';
import * as i0 from '@angular/core';
const _c0 = () => ['/home'];
const _c1 = () => ({ ref: 'component69' });
const _forTrack0 = ($index, $item) => $item.id;
const _forTrack1 = ($index, $item) => $item.id;
function Benchmark69Component_For_10_Template(rf, ctx) {
  if (rf & 1) {
    const _r1 = i0.ɵɵgetCurrentView();
    i0.ɵɵelementStart(0, 'li', 13);
    i0.ɵɵlistener('click', function Benchmark69Component_For_10_Template_li_click_0_listener() {
      const ctx_r1 = i0.ɵɵrestoreView(_r1);
      const item_r3 = ctx_r1.$implicit;
      const ctx_r3 = i0.ɵɵnextContext();
      return i0.ɵɵresetView(ctx_r3.select(item_r3));
    });
    i0.ɵɵelementStart(1, 'a', 14);
    i0.ɵɵtext(2);
    i0.ɵɵelementEnd()();
  }
  if (rf & 2) {
    const item_r3 = ctx.$implicit;
    const ctx_r3 = i0.ɵɵnextContext();
    i0.ɵɵclassProp('selected', item_r3.id === ctx_r3.selectedId);
    i0.ɵɵadvance();
    i0.ɵɵproperty('href', item_r3.url, i0.ɵɵsanitizeUrl);
    i0.ɵɵattribute('data-id', item_r3.id);
    i0.ɵɵadvance();
    i0.ɵɵtextInterpolate1(' ', item_r3.name, ' ');
  }
}
function Benchmark69Component_Conditional_12_Template(rf, ctx) {
  if (rf & 1) {
    i0.ɵɵelementStart(0, 'div', 15)(1, 'span');
    i0.ɵɵtext(2, 'Loading...');
    i0.ɵɵelementEnd()();
  }
  if (rf & 2) {
    const ctx_r4 = i0.ɵɵnextContext();
    i0.ɵɵstyleProp('width', ctx_r4.spinnerSize, 'px');
  }
}
function Benchmark69Component_Conditional_13_Template(rf, ctx) {
  if (rf & 1) {
    const _r6 = i0.ɵɵgetCurrentView();
    i0.ɵɵelementStart(0, 'div', 16)(1, 'p');
    i0.ɵɵtext(2);
    i0.ɵɵelementEnd();
    i0.ɵɵelementStart(3, 'button', 13);
    i0.ɵɵlistener(
      'click',
      function Benchmark69Component_Conditional_13_Template_button_click_3_listener() {
        i0.ɵɵrestoreView(_r6);
        const ctx_r6 = i0.ɵɵnextContext();
        return i0.ɵɵresetView(ctx_r6.retry());
      },
    );
    i0.ɵɵtext(4, 'Retry');
    i0.ɵɵelementEnd()();
  }
  if (rf & 2) {
    const ctx_r6 = i0.ɵɵnextContext();
    i0.ɵɵclassProp('critical', ctx_r6.isCritical);
    i0.ɵɵadvance(2);
    i0.ɵɵtextInterpolate(ctx_r6.errorMessage);
  }
}
function Benchmark69Component_Conditional_14_Template(rf, ctx) {
  if (rf & 1) {
    i0.ɵɵelementStart(0, 'section', 8);
    i0.ɵɵrepeaterCreate(
      1,
      Benchmark69Component_Conditional_14_For_2_Template,
      16,
      18,
      'article',
      17,
      _forTrack1,
    );
    i0.ɵɵelementEnd();
  }
  if (rf & 2) {
    const ctx_r13 = i0.ɵɵnextContext();
    i0.ɵɵadvance();
    i0.ɵɵrepeater(ctx_r13.data);
  }
}
function Benchmark69Component_Conditional_14_For_2_Template(rf, ctx) {
  if (rf & 1) {
    const _r8 = i0.ɵɵgetCurrentView();
    i0.ɵɵelementStart(0, 'article', 18);
    i0.ɵɵlistener(
      'mouseenter',
      function Benchmark69Component_Conditional_14_For_2_Template_article_mouseenter_0_listener() {
        const ctx_r8 = i0.ɵɵrestoreView(_r8);
        const row_r10 = ctx_r8.$implicit;
        const ctx_r10 = i0.ɵɵnextContext(2);
        return i0.ɵɵresetView(ctx_r10.onHover(row_r10));
      },
    );
    i0.ɵɵlistener(
      'mouseleave',
      function Benchmark69Component_Conditional_14_For_2_Template_article_mouseleave_0_listener() {
        const ctx_r8 = i0.ɵɵrestoreView(_r8);
        const row_r10 = ctx_r8.$implicit;
        const ctx_r10 = i0.ɵɵnextContext(2);
        return i0.ɵɵresetView(ctx_r10.onLeave(row_r10));
      },
    );
    i0.ɵɵelementStart(1, 'div', 19)(2, 'img', 20);
    i0.ɵɵelementEnd();
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
      function Benchmark69Component_Conditional_14_For_2_Template_button_click_12_listener() {
        const ctx_r8 = i0.ɵɵrestoreView(_r8);
        const row_r10 = ctx_r8.$implicit;
        const ctx_r10 = i0.ɵɵnextContext(2);
        return i0.ɵɵresetView(ctx_r10.edit(row_r10));
      },
    );
    i0.ɵɵtext(13, 'Edit');
    i0.ɵɵelementEnd();
    i0.ɵɵelementStart(14, 'button', 13);
    i0.ɵɵlistener(
      'click',
      function Benchmark69Component_Conditional_14_For_2_Template_button_click_14_listener() {
        const ctx_r8 = i0.ɵɵrestoreView(_r8);
        const row_r10 = ctx_r8.$implicit;
        const ctx_r10 = i0.ɵɵnextContext(2);
        return i0.ɵɵresetView(ctx_r10.delete(row_r10));
      },
    );
    i0.ɵɵtext(15, 'Delete');
    i0.ɵɵelementEnd()()();
  }
  if (rf & 2) {
    const row_r10 = ctx.$implicit;
    const ɵ$index_30_r12 = ctx.$index;
    const ɵ$count_30_r13 = ctx.$count;
    i0.ɵɵclassProp('first', ɵ$index_30_r12 === 0)('last', ɵ$index_30_r12 === ɵ$count_30_r13 - 1)(
      'even',
      ɵ$index_30_r12 % 2 === 0,
    );
    i0.ɵɵattribute('data-index', ɵ$index_30_r12);
    i0.ɵɵadvance(2);
    i0.ɵɵproperty('src', row_r10.image, i0.ɵɵsanitizeUrl)('alt', row_r10.title);
    i0.ɵɵadvance(2);
    i0.ɵɵtextInterpolate(row_r10.title);
    i0.ɵɵadvance(3);
    i0.ɵɵtextInterpolate(row_r10.description);
    i0.ɵɵadvance(2);
    i0.ɵɵtextInterpolate(i0.ɵɵpipeBind2(10, 15, row_r10.date, 'short'));
    i0.ɵɵadvance(3);
    i0.ɵɵproperty('disabled', row_r10.locked);
    i0.ɵɵadvance(2);
    i0.ɵɵclassProp('danger', true);
  }
}
function Benchmark69Component_div_16_Template(rf, ctx) {
  if (rf & 1) {
    i0.ɵɵelementStart(0, 'div', 25)(1, 'h4');
    i0.ɵɵtext(2);
    i0.ɵɵelementEnd();
    i0.ɵɵelementStart(3, 'div', 26);
    i0.ɵɵelementEnd()();
  }
  if (rf & 2) {
    const widget_r15 = ctx.$implicit;
    i0.ɵɵadvance(2);
    i0.ɵɵtextInterpolate(widget_r15.title);
    i0.ɵɵadvance();
    i0.ɵɵproperty('innerHTML', widget_r15.content, i0.ɵɵsanitizeHtml);
  }
}
export class Benchmark69Component {
  title = 'Benchmark Component 69';
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
  static ɵfac = function Benchmark69Component_Factory(t) {
    return new (t || Benchmark69Component)();
  };
  static ɵcmp = /* @__PURE__ */ i0.ɵɵdefineComponent({
    type: Benchmark69Component,
    selectors: [['app-benchmark-69']],
    decls: 22,
    vars: 15,
    consts: [
      [1, 'benchmark-component-69'],
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
    template: function Benchmark69Component_Template(rf, ctx) {
      if (rf & 1) {
        i0.ɵɵelementStart(0, 'div', 0)(1, 'header', 1);
        i0.ɵɵlistener(
          'click',
          function Benchmark69Component_Template_header_click_1_listener($event) {
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
        i0.ɵɵrepeaterCreate(9, Benchmark69Component_For_10_Template, 3, 5, 'li', 4, _forTrack0);
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(11, 'main', 5);
        i0.ɵɵconditionalCreate(12, Benchmark69Component_Conditional_12_Template, 3, 2, 'div', 6)(
          13,
          Benchmark69Component_Conditional_13_Template,
          5,
          3,
          'div',
          7,
        )(14, Benchmark69Component_Conditional_14_Template, 3, 0, 'section', 8);
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(15, 'aside', 9);
        i0.ɵɵtemplate(16, Benchmark69Component_div_16_Template, 4, 2, 'div', 10);
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
        i0.ɵɵtextInterpolate1('© ', ctx.currentYear, ' - Component 69');
        i0.ɵɵadvance();
        i0.ɵɵproperty('routerLink', i0.ɵɵpureFunction0(13, _c0))(
          'queryParams',
          i0.ɵɵpureFunction0(14, _c1),
        );
      }
    },
    standalone: true,
    styles: [],
    encapsulation: 2,
    inputs: {
      title: 'title',
      subtitle: 'subtitle',
    },
    outputs: { action: 'action' },
    dependencies: [CommonModule, i1.NgForOf, i1.UpperCasePipe, i1.DatePipe, RouterLink],
  });
}
