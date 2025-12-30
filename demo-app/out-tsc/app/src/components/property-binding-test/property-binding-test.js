import { NgIf, NgFor, NgClass, NgStyle } from '@angular/common';
import { Component } from '@angular/core';
import * as i0 from '@angular/core';
export class PropertyBindingTest {
  // Text bindings
  title = 'Property Binding Demo';
  description = 'Testing various property bindings';
  // Attribute bindings
  imageSrc = 'https://via.placeholder.com/150';
  imageAlt = 'Placeholder Image';
  linkHref = 'https://angular.io';
  // Boolean properties
  isDisabled = false;
  isReadonly = false;
  isHidden = false;
  // Style bindings
  textColor = 'blue';
  fontSize = 16;
  backgroundColor = '#f0f0f0';
  borderRadius = 8;
  // Class bindings
  isActive = false;
  isHighlighted = false;
  isPrimary = true;
  // Dynamic class object
  get dynamicClasses() {
    return {
      active: this.isActive,
      highlighted: this.isHighlighted,
      primary: this.isPrimary,
    };
  }
  // Dynamic style object
  get dynamicStyles() {
    return {
      color: this.textColor,
      'font-size': `${this.fontSize}px`,
      'background-color': this.backgroundColor,
      'border-radius': `${this.borderRadius}px`,
      padding: '10px',
    };
  }
  // Width/Height bindings
  boxWidth = 200;
  boxHeight = 100;
  // ARIA bindings
  ariaLabel = 'Interactive button';
  ariaExpanded = false;
  ariaDisabled = false;
  // Data attributes
  dataId = '12345';
  dataType = 'example';
  // Methods
  toggleDisabled() {
    this.isDisabled = !this.isDisabled;
  }
  toggleReadonly() {
    this.isReadonly = !this.isReadonly;
  }
  toggleHidden() {
    this.isHidden = !this.isHidden;
  }
  toggleActive() {
    this.isActive = !this.isActive;
  }
  toggleHighlighted() {
    this.isHighlighted = !this.isHighlighted;
  }
  togglePrimary() {
    this.isPrimary = !this.isPrimary;
  }
  setColor(color) {
    this.textColor = color;
  }
  increaseFontSize() {
    this.fontSize += 2;
  }
  decreaseFontSize() {
    if (this.fontSize > 8) {
      this.fontSize -= 2;
    }
  }
  toggleAriaExpanded() {
    this.ariaExpanded = !this.ariaExpanded;
  }
  static ɵfac = function PropertyBindingTest_Factory(__ngFactoryType__) {
    return new (__ngFactoryType__ || PropertyBindingTest)();
  };
  static ɵcmp = /*@__PURE__*/ i0.ɵɵdefineComponent({
    type: PropertyBindingTest,
    selectors: [['app-property-binding-test']],
    decls: 82,
    vars: 40,
    consts: [
      [1, 'property-binding-test'],
      ['target', '_blank', 3, 'href'],
      [3, 'click'],
      ['type', 'text', 'placeholder', 'Disabled test', 'value', 'Test', 3, 'disabled'],
      ['type', 'text', 'placeholder', 'Readonly test', 'value', 'Readonly', 3, 'readonly'],
      [3, 'hidden'],
      [1, 'styled-box'],
      [3, 'ngStyle'],
      [1, 'box'],
      [1, 'box', 3, 'ngClass'],
    ],
    template: function PropertyBindingTest_Template(rf, ctx) {
      if (rf & 1) {
        i0.ɵɵelementStart(0, 'div', 0)(1, 'h2');
        i0.ɵɵtext(2);
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(3, 'p');
        i0.ɵɵtext(4);
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(5, 'section')(6, 'h3');
        i0.ɵɵtext(7, '1. Basic Property Binding [property]');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(8, 'a', 1);
        i0.ɵɵtext(9, 'Angular Website');
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(10, 'section')(11, 'h3');
        i0.ɵɵtext(12, '2. Boolean Property Bindings');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(13, 'div')(14, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_14_listener() {
          return ctx.toggleDisabled();
        });
        i0.ɵɵtext(15, 'Toggle Disabled');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(16, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_16_listener() {
          return ctx.toggleReadonly();
        });
        i0.ɵɵtext(17, 'Toggle Readonly');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(18, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_18_listener() {
          return ctx.toggleHidden();
        });
        i0.ɵɵtext(19, 'Toggle Hidden');
        i0.ɵɵelementEnd()();
        i0.ɵɵelement(20, 'input', 3)(21, 'input', 4);
        i0.ɵɵelementStart(22, 'p', 5);
        i0.ɵɵtext(23, 'This paragraph can be hidden');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(24, 'p');
        i0.ɵɵtext(25);
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(26, 'section')(27, 'h3');
        i0.ɵɵtext(28, '3. Style Binding [style.property]');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(29, 'div')(30, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_30_listener() {
          return ctx.setColor('red');
        });
        i0.ɵɵtext(31, 'Red');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(32, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_32_listener() {
          return ctx.setColor('green');
        });
        i0.ɵɵtext(33, 'Green');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(34, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_34_listener() {
          return ctx.setColor('blue');
        });
        i0.ɵɵtext(35, 'Blue');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(36, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_36_listener() {
          return ctx.increaseFontSize();
        });
        i0.ɵɵtext(37, 'Font +');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(38, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_38_listener() {
          return ctx.decreaseFontSize();
        });
        i0.ɵɵtext(39, 'Font -');
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(40, 'p');
        i0.ɵɵtext(41, ' This text has dynamic color and font size ');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(42, 'div', 6);
        i0.ɵɵtext(43, ' Dynamic box ');
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(44, 'section')(45, 'h3');
        i0.ɵɵtext(46, '4. [ngStyle] Object Binding');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(47, 'p', 7);
        i0.ɵɵtext(48, 'This text uses ngStyle with an object');
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(49, 'section')(50, 'h3');
        i0.ɵɵtext(51, '5. Class Binding [class.name]');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(52, 'div')(53, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_53_listener() {
          return ctx.toggleActive();
        });
        i0.ɵɵtext(54, 'Toggle Active');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(55, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_55_listener() {
          return ctx.toggleHighlighted();
        });
        i0.ɵɵtext(56, 'Toggle Highlighted');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(57, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_57_listener() {
          return ctx.togglePrimary();
        });
        i0.ɵɵtext(58, 'Toggle Primary');
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(59, 'div', 8);
        i0.ɵɵtext(60, ' Class-bound box ');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(61, 'p');
        i0.ɵɵtext(62);
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(63, 'section')(64, 'h3');
        i0.ɵɵtext(65, '6. [ngClass] Object Binding');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(66, 'div', 9);
        i0.ɵɵtext(67, 'ngClass bound box');
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(68, 'section')(69, 'h3');
        i0.ɵɵtext(70, '7. Attribute Binding [attr.name]');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(71, 'button', 2);
        i0.ɵɵlistener('click', function PropertyBindingTest_Template_button_click_71_listener() {
          return ctx.toggleAriaExpanded();
        });
        i0.ɵɵtext(72, ' ARIA Bound Button ');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(73, 'p');
        i0.ɵɵtext(74);
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(75, 'div');
        i0.ɵɵtext(76, 'Element with data attribute');
        i0.ɵɵelementEnd()();
        i0.ɵɵelementStart(77, 'section')(78, 'h3');
        i0.ɵɵtext(79, '8. Conditional Attribute (null removes)');
        i0.ɵɵelementEnd();
        i0.ɵɵelementStart(80, 'button');
        i0.ɵɵtext(81, 'Conditionally Disabled');
        i0.ɵɵelementEnd()()();
      }
      if (rf & 2) {
        i0.ɵɵadvance(2);
        i0.ɵɵtextInterpolate(ctx.title);
        i0.ɵɵadvance(2);
        i0.ɵɵtextInterpolate(ctx.description);
        i0.ɵɵadvance(4);
        i0.ɵɵproperty('href', ctx.linkHref, i0.ɵɵsanitizeUrl);
        i0.ɵɵadvance(12);
        i0.ɵɵproperty('disabled', ctx.isDisabled);
        i0.ɵɵadvance();
        i0.ɵɵproperty('readonly', ctx.isReadonly);
        i0.ɵɵadvance();
        i0.ɵɵproperty('hidden', ctx.isHidden);
        i0.ɵɵadvance(3);
        i0.ɵɵtextInterpolate3(
          'Disabled: ',
          ctx.isDisabled,
          ', Readonly: ',
          ctx.isReadonly,
          ', Hidden: ',
          ctx.isHidden,
        );
        i0.ɵɵadvance(15);
        i0.ɵɵstyleProp('color', ctx.textColor)('font-size', ctx.fontSize, 'px');
        i0.ɵɵadvance(2);
        i0.ɵɵstyleProp('width', ctx.boxWidth, 'px')('height', ctx.boxHeight, 'px')(
          'background-color',
          ctx.backgroundColor,
        )('border-radius', ctx.borderRadius, 'px');
        i0.ɵɵadvance(5);
        i0.ɵɵproperty('ngStyle', ctx.dynamicStyles);
        i0.ɵɵadvance(12);
        i0.ɵɵclassProp('active', ctx.isActive)('highlighted', ctx.isHighlighted)(
          'primary',
          ctx.isPrimary,
        );
        i0.ɵɵadvance(3);
        i0.ɵɵtextInterpolate3(
          'Active: ',
          ctx.isActive,
          ', Highlighted: ',
          ctx.isHighlighted,
          ', Primary: ',
          ctx.isPrimary,
        );
        i0.ɵɵadvance(4);
        i0.ɵɵproperty('ngClass', ctx.dynamicClasses);
        i0.ɵɵadvance(5);
        i0.ɵɵattribute('aria-label', ctx.ariaLabel)('aria-expanded', ctx.ariaExpanded)(
          'aria-disabled',
          ctx.ariaDisabled,
        )('data-id', ctx.dataId)('data-type', ctx.dataType);
        i0.ɵɵadvance(3);
        i0.ɵɵtextInterpolate1('aria-expanded: ', ctx.ariaExpanded);
        i0.ɵɵadvance();
        i0.ɵɵattribute('data-custom', 'custom-value-' + ctx.dataId);
        i0.ɵɵadvance(5);
        i0.ɵɵattribute('disabled', ctx.isDisabled ? '' : null);
      }
    },
    dependencies: [NgClass, NgStyle],
    styles: [
      '.property-binding-test[_ngcontent-%COMP%] {\n  padding: 20px;\n  font-family: Arial, sans-serif;\n}\n\nsection[_ngcontent-%COMP%] {\n  margin-bottom: 24px;\n  padding: 16px;\n  border: 1px solid #ddd;\n  border-radius: 8px;\n}\n\nh3[_ngcontent-%COMP%] {\n  margin-top: 0;\n  color: #333;\n}\n\nbutton[_ngcontent-%COMP%] {\n  margin-right: 8px;\n  margin-bottom: 8px;\n  padding: 8px 16px;\n  cursor: pointer;\n}\n\ninput[_ngcontent-%COMP%] {\n  display: block;\n  margin: 8px 0;\n  padding: 8px;\n}\n\n.styled-box[_ngcontent-%COMP%] {\n  display: flex;\n  align-items: center;\n  justify-content: center;\n  margin-top: 16px;\n  color: #333;\n}\n\n.box[_ngcontent-%COMP%] {\n  padding: 16px;\n  margin: 8px 0;\n  border: 2px solid #ccc;\n  transition: all 0.3s ease;\n}\n\n.box.active[_ngcontent-%COMP%] {\n  border-color: #2196f3;\n  background-color: #e3f2fd;\n}\n\n.box.highlighted[_ngcontent-%COMP%] {\n  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);\n}\n\n.box.primary[_ngcontent-%COMP%] {\n  font-weight: bold;\n  color: #1976d2;\n}\n\nimg[_ngcontent-%COMP%] {\n  margin-right: 16px;\n}',
    ],
  });
}
(() => {
  (typeof ngDevMode === 'undefined' || ngDevMode) &&
    i0.ɵsetClassMetadata(
      PropertyBindingTest,
      [
        {
          type: Component,
          args: [
            {
              selector: 'app-property-binding-test',
              standalone: true,
              imports: [NgIf, NgFor, NgClass, NgStyle],
              template:
                '<div class="property-binding-test">\n  <h2>{{ title }}</h2>\n  <p>{{ description }}</p>\n\n  <!-- Test 1: Property Binding -->\n  <section>\n    <h3>1. Basic Property Binding [property]</h3>\n    <!-- <img [src]="imageSrc" [alt]="imageAlt" width="100" /> -->\n    <a [href]="linkHref" target="_blank">Angular Website</a>\n  </section>\n\n  <!-- Test 2: Disabled/Readonly Properties -->\n  <section>\n    <h3>2. Boolean Property Bindings</h3>\n    <div>\n      <button (click)="toggleDisabled()">Toggle Disabled</button>\n      <button (click)="toggleReadonly()">Toggle Readonly</button>\n      <button (click)="toggleHidden()">Toggle Hidden</button>\n    </div>\n    <input type="text" [disabled]="isDisabled" placeholder="Disabled test" value="Test" />\n    <input type="text" [readonly]="isReadonly" placeholder="Readonly test" value="Readonly" />\n    <p [hidden]="isHidden">This paragraph can be hidden</p>\n    <p>Disabled: {{ isDisabled }}, Readonly: {{ isReadonly }}, Hidden: {{ isHidden }}</p>\n  </section>\n\n  <!-- Test 3: Style Binding -->\n  <section>\n    <h3>3. Style Binding [style.property]</h3>\n    <div>\n      <button (click)="setColor(\'red\')">Red</button>\n      <button (click)="setColor(\'green\')">Green</button>\n      <button (click)="setColor(\'blue\')">Blue</button>\n      <button (click)="increaseFontSize()">Font +</button>\n      <button (click)="decreaseFontSize()">Font -</button>\n    </div>\n    <p [style.color]="textColor" [style.font-size.px]="fontSize">\n      This text has dynamic color and font size\n    </p>\n    <div\n      [style.width.px]="boxWidth"\n      [style.height.px]="boxHeight"\n      [style.background-color]="backgroundColor"\n      [style.border-radius.px]="borderRadius"\n      class="styled-box"\n    >\n      Dynamic box\n    </div>\n  </section>\n\n  <!-- Test 4: ngStyle Binding -->\n  <section>\n    <h3>4. [ngStyle] Object Binding</h3>\n    <p [ngStyle]="dynamicStyles">This text uses ngStyle with an object</p>\n  </section>\n\n  <!-- Test 5: Class Binding -->\n  <section>\n    <h3>5. Class Binding [class.name]</h3>\n    <div>\n      <button (click)="toggleActive()">Toggle Active</button>\n      <button (click)="toggleHighlighted()">Toggle Highlighted</button>\n      <button (click)="togglePrimary()">Toggle Primary</button>\n    </div>\n    <div\n      class="box"\n      [class.active]="isActive"\n      [class.highlighted]="isHighlighted"\n      [class.primary]="isPrimary"\n    >\n      Class-bound box\n    </div>\n    <p>Active: {{ isActive }}, Highlighted: {{ isHighlighted }}, Primary: {{ isPrimary }}</p>\n  </section>\n\n  <!-- Test 6: ngClass Binding -->\n  <section>\n    <h3>6. [ngClass] Object Binding</h3>\n    <div class="box" [ngClass]="dynamicClasses">ngClass bound box</div>\n  </section>\n\n  <!-- Test 7: Attribute Binding -->\n  <section>\n    <h3>7. Attribute Binding [attr.name]</h3>\n    <button\n      [attr.aria-label]="ariaLabel"\n      [attr.aria-expanded]="ariaExpanded"\n      [attr.aria-disabled]="ariaDisabled"\n      [attr.data-id]="dataId"\n      [attr.data-type]="dataType"\n      (click)="toggleAriaExpanded()"\n    >\n      ARIA Bound Button\n    </button>\n    <p>aria-expanded: {{ ariaExpanded }}</p>\n    <div [attr.data-custom]="\'custom-value-\' + dataId">Element with data attribute</div>\n  </section>\n\n  <!-- Test 8: Conditional Attribute (null removes attribute) -->\n  <section>\n    <h3>8. Conditional Attribute (null removes)</h3>\n    <button [attr.disabled]="isDisabled ? \'\' : null">Conditionally Disabled</button>\n  </section>\n</div>\n',
              styles: [
                '.property-binding-test {\n  padding: 20px;\n  font-family: Arial, sans-serif;\n}\n\nsection {\n  margin-bottom: 24px;\n  padding: 16px;\n  border: 1px solid #ddd;\n  border-radius: 8px;\n}\n\nh3 {\n  margin-top: 0;\n  color: #333;\n}\n\nbutton {\n  margin-right: 8px;\n  margin-bottom: 8px;\n  padding: 8px 16px;\n  cursor: pointer;\n}\n\ninput {\n  display: block;\n  margin: 8px 0;\n  padding: 8px;\n}\n\n.styled-box {\n  display: flex;\n  align-items: center;\n  justify-content: center;\n  margin-top: 16px;\n  color: #333;\n}\n\n.box {\n  padding: 16px;\n  margin: 8px 0;\n  border: 2px solid #ccc;\n  transition: all 0.3s ease;\n}\n\n.box.active {\n  border-color: #2196f3;\n  background-color: #e3f2fd;\n}\n\n.box.highlighted {\n  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);\n}\n\n.box.primary {\n  font-weight: bold;\n  color: #1976d2;\n}\n\nimg {\n  margin-right: 16px;\n}\n',
              ],
            },
          ],
        },
      ],
      null,
      null,
    );
})();
(() => {
  (typeof ngDevMode === 'undefined' || ngDevMode) &&
    i0.ɵsetClassDebugInfo(PropertyBindingTest, {
      className: 'PropertyBindingTest',
      filePath: 'src/app/src/components/property-binding-test/property-binding-test.ts',
      lineNumber: 11,
    });
})();
