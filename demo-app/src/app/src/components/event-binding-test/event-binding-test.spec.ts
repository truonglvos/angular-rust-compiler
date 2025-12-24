import { ComponentFixture, TestBed } from '@angular/core/testing';

import { EventBindingTest } from './event-binding-test';

describe('EventBindingTest', () => {
  let component: EventBindingTest;
  let fixture: ComponentFixture<EventBindingTest>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [EventBindingTest]
    })
    .compileComponents();

    fixture = TestBed.createComponent(EventBindingTest);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
