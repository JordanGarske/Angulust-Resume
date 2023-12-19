import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, of } from 'rxjs';
import { catchError, map, tap } from 'rxjs/operators';
import {User} from '../models/user';
// angular models
@Injectable({
  providedIn: 'root'
})
export class UserService {
  private  urlUser = "http://0.0.0.0:8080/";
  httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' })
  };

  constructor(private http: HttpClient) { }
  addNewUser(newUser: User): Observable<User>{
    return this.http.post<User>(`${this.urlUser}sign-up`, newUser, this.httpOptions);
  }
  loginUser(user: User): Observable<boolean>{
    return this.http.post<boolean>(`${this.urlUser}login`, user, this.httpOptions);
  }
}
