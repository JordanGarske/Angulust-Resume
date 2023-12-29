import { Component } from '@angular/core';
import { UserService } from '../services/user.service';
import { Router } from '@angular/router';
import { User } from '../models/user';

@Component({
  selector: 'app-admin',
  templateUrl: './admin.component.html',
  styleUrls: ['./admin.component.scss']
})
export class AdminComponent {
users: User[] = [];
  constructor(private userService: UserService, private router: Router){}

  ngOnInit(): void{
    this.userService.getUsers().subscribe(results => this.users = results);
  }
  deleteUserButton():void{

  }
}
