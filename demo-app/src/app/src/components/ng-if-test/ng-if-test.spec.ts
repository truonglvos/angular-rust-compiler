import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NgIfTest } from './ng-if-test';

describe('NgIfTest', () => {
  let component: NgIfTest;
  let fixture: ComponentFixture<NgIfTest>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [NgIfTest],
    }).compileComponents();

    fixture = TestBed.createComponent(NgIfTest);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
