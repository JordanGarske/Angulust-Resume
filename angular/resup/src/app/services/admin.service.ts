import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { ClientReference } from '../models/reference';
import { Observable } from 'rxjs';
import { Client, ClientReview } from '../models/admin/Client';
import { Review } from '../models/admin/Review';
@Injectable({
  providedIn: 'root'
})
export class AdminService {
  private  urlUser = "https://jordan-garske.com/admin/";
  public isAdmin:boolean = false;
  private canWriteReference:boolean=  false;
  public referenceID:number = -1;
  httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' })
  };
  constructor(private http: HttpClient) { }
  getReferences(): Observable<ClientReview[]>{
    return this.http.get<ClientReview[]>(`${this.urlUser}review/get`);
  }
  getClients(): Observable<Client[]>{
    return this.http.get<Client[]>(`${this.urlUser}client/get`);
  }
  addReviewPermission(userID:number):Observable<[boolean, Review]>{
    return this.http.get<[boolean, Review]>(`${this.urlUser}reference_permission_access/${userID}`);
  }  
  deleteReview(review_id: number):Observable<boolean>{
    return this.http.get<boolean>(`${this.urlUser}review/delete/${review_id}`);
  }
}
