import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { ClientReference } from 'src/app/models/reference';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent {
  references:ClientReference[] = []
  constructor(private userService: UserService, private router: Router){}
  ngOnInit(): void{
    this.userService.getReference().subscribe(results => this.references = results);
  }
}
