import { Component } from '@angular/core';

import { HeaderComponent } from "../header/header.component";
import { HeroComponent } from "../../../shared/components/hero/hero.component";

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [HeaderComponent, HeroComponent],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css'
})
export class HomeComponent {

}
