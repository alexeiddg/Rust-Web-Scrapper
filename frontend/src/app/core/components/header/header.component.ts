import { Component } from '@angular/core';
import { MatButtonModule } from "@angular/material/button";

import { GithubIconComponent } from "../../../shared/assets/icons/github-icon/github-icon.component";

@Component({
  selector: 'app-header',
  standalone: true,
  imports: [ MatButtonModule, GithubIconComponent ],
  templateUrl: './header.component.html',
  styleUrl: './header.component.css'
})
export class HeaderComponent {

}
