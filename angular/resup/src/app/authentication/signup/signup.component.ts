import { Component } from '@angular/core';
import { UserService } from 'src/app/services/user.service';
import {User, UserSignUp} from '../../models/user';
import { Router } from '@angular/router';
@Component({
  selector: 'app-signup',
  templateUrl: './signup.component.html',
  styleUrls: ['./signup.component.scss']
})
export class SignupComponent {
   userData: UserSignUp= {
     first_name: '',
     last_name: '',
     client_password: '',
     email: '',
     admin_privilege: false,
     phone_number: '',
     profession: '',
     company: ''
   };
   createdUser?:User;
  constructor(private userService: UserService, private router: Router){}
  onSubmit() {
    if(this.userData.phone_number == ''){
      this.userData.phone_number = undefined;
    }
    this.userService.addNewUser(this.userData).subscribe(user => this.createdUser = user );
  }
  signup(): void {
    
    this.router.navigate(["/login"]);
  }
}