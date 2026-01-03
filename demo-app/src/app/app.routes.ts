import { Routes } from '@angular/router';
import { ButtonTestComponent } from './src/components/materials/button/button';
import { AutocompleteTestComponent } from './src/components/materials/autocomplete/autocomplete';
import { InputTestComponent } from './src/components/materials/input/input';

export const routes: Routes = [
  {
    path: 'button',
    component: ButtonTestComponent,
  },
  {
    path: 'autocomplete',
    component: AutocompleteTestComponent,
  },
  {
    path: 'input',
    component: InputTestComponent,
  },
  {
    path: '**',
    redirectTo: 'button',
  },
];
