import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class ChatRoomService {

  private  urlUser = "https://jordan-garske.com/";
  private eventSource: EventSource  = new EventSource(this.urlUser);
  httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' })
  };
  constructor(private http: HttpClient) { }
  connect(): Observable<MessageEvent<string>>{
    return new Observable(observe =>{
      this.eventSource = new EventSource(`${this.urlUser}events`);

      this.eventSource.onmessage = (event: MessageEvent) => {
        observe.next(event);
      };

      this.eventSource.onerror = (error) => {
        observe.error(error);
        this.eventSource.close();
      };

      return () => {
        this.eventSource.close();
      };

    })
  }
  send_message(message: Message): Observable<MessageEvent<Message>>{
    return this.http.post<MessageEvent<Message>>(`${this.urlUser}message`, message, this.httpOptions);
  }
  get_messages(): Observable<Message[]>{
    return this.http.get<Message[]>(`${this.urlUser}messages`, this.httpOptions);
  }
}
export interface Message {
  client_id: number;
  room_id: number,
  username: string;
  message: string;
}
