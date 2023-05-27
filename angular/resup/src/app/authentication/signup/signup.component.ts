import { Component } from '@angular/core';

@Component({
  selector: 'app-signup',
  templateUrl: './signup.component.html',
  styleUrls: ['./signup.component.scss']
})
export class SignupComponent {
  formData = {
    firstName: '',
    lastName: '',
    password: '',
    email: ''
  };

  onSubmit() {
    console.log(this.formData);
    // Perform sign-up logic here, such as sending the form data to a server
  }
}
