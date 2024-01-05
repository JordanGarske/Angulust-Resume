import { Component } from '@angular/core';
import { UserService } from '../services/user.service';
import { AdminService } from '../services/admin.service';
import { Router } from '@angular/router';
import { User } from '../models/user';
import { Client, ClientReview } from '../models/admin/Client';
import { Review } from '../models/admin/Review';

@Component({
  selector: 'app-admin',
  templateUrl: './admin.component.html',
  styleUrls: ['./admin.component.scss'],
})
export class AdminComponent {
  clients: Client[] = [];
  references: ClientReview[] = [];
  didUpdate: string = '';
  canDelete: boolean = false;
  constructor(
    private userService: UserService,
    private adminService: AdminService,
    private router: Router
  ) {}

  ngOnInit(): void {
    this.adminService.getReferences().subscribe((results) =>this.references = results);
    this.adminService.getClients().subscribe((results) => this.clients = results);
  }
  // deleteUserButton(user: User): void {
  //   this.adminService.referenceFormPermission(user.id).subscribe((result) => {
  //     this.updateResult(result, user.id);
  //   });
  // }
  addReviewButton(user:Client ) {
    this.didUpdate = '';
    this.adminService.addReviewPermission(user.id).subscribe(result =>{
      if(result[0]){
        this.references.push(this.mapClientAndReview(user,result[1]));
      }
    });
  }
  deleteReviewButton(user: ClientReview) {
    this.adminService.deleteReview(user.review_id).subscribe((results) => this.updateResult(results, user.review_id));
  }
  updateResult(result: boolean, id: number): void {
    if (result) {
      this.didUpdate = 'succes';
      this.references = this.references.filter((user) => user.review_id !== id);
    } else {
      this.didUpdate = 'fail';
    }
  }
  mapClientAndReview(cli: Client, rev: Review): ClientReview{
     let x: ClientReview = {
       id: cli.id,
       email: cli.email,
       first_name: cli.first_name,
       last_name: cli.last_name,
       profession: cli.profession,
       company: cli.company,
       elucidation: rev.elucidation,
       review_id: rev.id,
       client_password: cli.client_password,
       admin_privilege: cli.admin_privilege,
       phone_number: cli.phone_number
     };
     return x; 
  }
}
