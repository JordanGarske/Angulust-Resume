import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, of } from 'rxjs';
import { catchError, map, tap } from 'rxjs/operators';
import {User, UserLogin, UserSignUp} from '../models/user';
import { CredentialApproval } from '../models/credential';
import { ClientReference, ResumeReference } from '../models/reference';
// angular models
@Injectable({
  providedIn: 'root'
})
//"https://jordan-garske.com/"
//"http://127.0.0.1:8000/"
export class UserService {
  private  urlUser = "https://jordan-garske.com/";
  public isAdmin:boolean = false;
  private canWriteReference:boolean=  false;
  public referenceID:number = -1;
  httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' })
  };
  constructor(private http: HttpClient) { }
  writeReview(ref: ResumeReference): Observable<boolean>{
    return this.http.post<boolean>(`${this.urlUser}client_access/write_my_review`, ref, this.httpOptions);
  }
  addNewUser(newUser: UserSignUp): Observable<boolean>{
    return this.http.post<boolean>(`${this.urlUser}sign-up`, newUser, this.httpOptions);
  }
  loginUser(user: UserLogin): Observable<CredentialApproval>{
    return this.http.post<CredentialApproval>(`${this.urlUser}login`, user, this.httpOptions);
  }
  //crud 
  getReference(): Observable<ClientReference[]>{
    return this.http.get<ClientReference[]>(`${this.urlUser}client_access/gather_reviews`);
  }
  //helper 
  canWrite(isItANumber:number|null):void{
    if(isItANumber === null){
      this.canWriteReference = false
    }
    else{
      this.referenceID = isItANumber
      this.canWriteReference = true 
    }
  }
  getCanWriteReference(): boolean{
    return this.canWriteReference;
  }
}
export interface ID {
  id: number;
}
