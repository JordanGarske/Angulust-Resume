import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, of } from 'rxjs';
import { catchError, map, tap } from 'rxjs/operators';
import {User, UserLogin, UserSignUp} from '../models/user';
import { CredentialApproval } from '../models/credential';
// angular models
@Injectable({
  providedIn: 'root'
})
export class UserService {
  private  urlUser = "http://127.0.0.1:8000/";
  public isAdmin:boolean = false;
  public canWriteReference:boolean = false;
  httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' })
  };

  constructor(private http: HttpClient) { }
  addNewUser(newUser: UserSignUp): Observable<User>{
    return this.http.post<User>(`${this.urlUser}sign-up`, newUser, this.httpOptions);
  }
  loginUser(user: UserLogin): Observable<CredentialApproval>{
    return this.http.post<CredentialApproval>(`${this.urlUser}login`, user, this.httpOptions);
  }
  //crud 
  getUsers(): Observable<User[]>{
    return this.http.get<User[]>(`${this.urlUser}admin/get_users`);
  }
  getLogs(user: User): Observable<User[]>{
    return this.http.get<User[]>(`${this.urlUser}admin/get_logs`);
  }
  deleteUser(userID: number): Observable<boolean>{
    return this.http.post<boolean>(`${this.urlUser}admin/delete_user`, userID, this.httpOptions);
  }
  referenceFormPermission(userID:number):Observable<boolean>{
    return this.http.post<boolean>(`${this.urlUser}admin/reference_permission_access`, userID, this.httpOptions);
  }  
}
