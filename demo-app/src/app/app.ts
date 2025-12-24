import { NgFor } from '@angular/common';
import {
  ChangeDetectionStrategy,
  Component,
  EventEmitter,
  inject,
  input,
  Input,
  OnInit,
  output,
  Output,
  signal,
} from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { RouterOutlet } from '@angular/router';
import { FullNamePipe } from './src/pipes/full-name-pipe';
import { NgForTest } from './src/components/ng-for/ng-for';
import { NgIfTest } from './src/components/ng-if-test/ng-if-test';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, NgFor, FullNamePipe, NgForTest, NgIfTest],
  templateUrl: './app.html',
  styleUrl: './app.css',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class App implements OnInit {
  @Input() fullName = 'Le Van Trường';

  header = input<string>('header 1');

  header2 = input.required<string>();

  @Output() nameChange = new EventEmitter<string>();

  haderChange = output<string>();

  surname = 'Lê';
  name = 'Trường';

  protected readonly title = signal('demo-app 5');
  protected readonly items = signal([
    { title: 'Item 9', link: 'https://example.com/item1' },
    { title: 'Item 2', link: 'https://example.com/item2' },
    { title: 'Item 3', link: 'https://example.com/item3' },
  ]);

  protected readonly fb = inject(FormBuilder);

  protected form!: FormGroup;

  ngOnInit(): void {
    this.form = this.fb.group({
      name: ['string', Validators.required],
    });
  }
}
