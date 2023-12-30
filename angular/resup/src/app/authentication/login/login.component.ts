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
              this.userService.isAdmin = result.admin;
              this.userService.canWriteReference = result.can_write_reference
              if(result.admin){
                this.router.navigate(["/admin"]);
              }
              else{
                this.router.navigate(["/home"]);
            }

           }
           else{
              //set back to defualt value for user to try agian to login
              userData: this.userData= {client_password: '',email: ''};
           }

         });
  }
  signup(): void {

      this.router.navigate(["/signup"]);
  }

}

