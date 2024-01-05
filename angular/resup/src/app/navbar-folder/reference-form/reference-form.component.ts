import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { ResumeReference } from 'src/app/models/reference';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-reference-form',
  templateUrl: './reference-form.component.html',
  styleUrls: ['./reference-form.component.scss']
})
export class ReferenceFormComponent {
  userData: ResumeReference= {client_id: 0,elucidation: ''};
  constructor(private userService: UserService, private router: Router){}
  submitWriting(){
    this.userData.client_id = this.userService.referenceID
    this.userService.writeReview(this.userData).subscribe( );
  }
}
