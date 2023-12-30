import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { User, UserLogin } from 'src/app/models/user';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent {
  constructor(private userService: UserService, private router: Router){}
  userData: UserLogin= {
    client_password: '',
    email: '',
  };
  onSubmit() {
    this.userService.loginUser(this.userData).subscribe(result => {    
          if(result.approved ){
             
           }
           else if(result.admin){
            this.router.navigate(["/admin"]);
            
           }
           else{
            this.router.navigate(["/home"]);
           }
         });
  }
  signup(): void {

      this.router.navigate(["/signup"]);
  }

}

