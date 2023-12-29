import { Component } from '@angular/core';
import { UserService } from '../services/user.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrls: ['./navbar.component.scss']
})
export class NavbarComponent {
  constructor(private userService: UserService, private router: Router){}
  roadmap_button(): void {

    this.router.navigate(["/home/roadmap"]);
}
}
