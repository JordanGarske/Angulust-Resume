import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { User } from 'src/app/models/user';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent {
  constructor(private userService: UserService, private router: Router){}
  userData: User= {
    client_id: 1,
    first_name: '',
    last_name: '',
    client_password: '',
    email: '',
    admin_privilege: false
  };
  createdUser?:User;
  onSubmit() {
    this.userService.loginUser(this.userData).subscribe(result => {    
          if(result){
             this.router.navigate(["/home"]);
           }
           else{
            this.userData.client_password= "";
            this.userData.email= "";
           }
         });
  }
  signup(): void {

      this.router.navigate(["/signup"]);
  }

}

