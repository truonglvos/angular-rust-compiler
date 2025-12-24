import { Component } from '@angular/core';

@Component({
  selector: 'app-event-binding-test',
  imports: [],
  templateUrl: './event-binding-test.html',
  styleUrl: './event-binding-test.css',
})
export class EventBindingTest {
  clickCount = 0;
  onClick(){
    this.clickCount++;
  }
}
