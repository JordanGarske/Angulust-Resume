import { Component } from '@angular/core';
import { UserService } from '../services/user.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrls: ['./navbar.component.scss']
})
export class NavbarComponent {
  canWriteRef:boolean = false;
  constructor(private userService: UserService, private router: Router){}
  ngOnInit(): void{
      this.canWriteRef = this.userService.getCanWriteReference();
  }
  roadmapButton(): void {
    this.router.navigate(["/home/roadmap"]);
  }
  referenceButton(): void {
    this.router.navigate(["/home/reference"]);
  }
}
