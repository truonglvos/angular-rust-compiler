import { NgFor } from '@angular/common';
import { ChangeDetectionStrategy, Component } from '@angular/core';

@Component({
  selector: 'app-ng-for',
  imports: [NgFor],
  templateUrl: './ng-for.html',
  styleUrl: './ng-for.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class NgForTest {
  protected readonly items = ['item 1', 'item 2', 'item 3'];
}
