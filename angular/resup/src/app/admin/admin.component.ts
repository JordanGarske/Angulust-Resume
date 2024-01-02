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
didUpdate:string = '';
  constructor(private userService: UserService, private router: Router){}

  ngOnInit(): void{
    this.userService.getUsers().subscribe(results => {
      this.users = results;
      this.userService.isAdmin = true;
    });
  }
  deleteUserButton(user: number):void{
    this.userService.deleteUser(user).subscribe()
  }
  referencePermissionButton(user: number){
    this.didUpdate = '';
    this.userService.referenceFormPermission(user).subscribe(result=>{
        this.updateResult(result, user)
        
    }
      );
  }
  updateResult(result: boolean, id: number ):void{
      if(result){
        this.didUpdate = "succes"
        this.users.filter(user => user.id !== id )
      }
      else{
        this.didUpdate = "fail"
      }
  }
}
