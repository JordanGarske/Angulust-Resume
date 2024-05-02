import { Component, OnInit, ChangeDetectorRef } from '@angular/core';
import { ChatRoomService, Message } from '../services/chat-room.service';
import { UserService } from '../services/user.service';

@Component({
  selector: 'app-chat-room',
  templateUrl: './chat-room.component.html',
  styleUrls: ['./chat-room.component.scss']
})
export class ChatRoomComponent  implements OnInit {
  status:string = "connecting"
  messages: Message[] = [];
  newMessage: Message = {
    room_id: 1,
    client_id: 1,
    username: '', 
    message: '',

  };
  constructor(private roomService: ChatRoomService, private userService: UserService,private changeDetection: ChangeDetectorRef){}
  ngOnInit() {
    this.init();
  }

  init() {
    // Initialize some rooms.
    this.addMessage('Rocket', 'Hey! Open another browser tab, send a message.');
    this.addMessage('Rocket', 'This is another room. Neat, huh?');
    this.addMessage('Rocket', `Look, your own " room! Nice.`);
    this.roomService.get_messages().subscribe(messages =>
      this.messages = messages
      );

    // Subscribe to server-sent events.
    this.subscribe();
  }

  getState() {
    return {
      client_room_id: 1,
      room: 'lobby',
      rooms: {},
      connected: false,
    };
  }



  addMessage( username: string, message: string): void {
    const state = this.getState();
    let mess: Message = {
      room_id: 1,
      client_id: 1,
      username: username, 
      message: message,
      };
    this.messages.push(mess);
  }
  sendMessage(){
    this.roomService.send_message(this.newMessage).subscribe();
  }

  subscribe(): void {
    const state = this.getState();
    this.roomService.connect().subscribe(events =>{

       let userObject: Message = JSON.parse(events.data);
       this.messages.push(userObject);
       this.changeDetection.detectChanges();
    });

  }

  setConnectedStatus(status: boolean): void {
    const state = this.getState();
    state.connected = status;
    if (status) {
      this.status = 'connected';
    } else {
      this.status  = 'reconnecting';
    }
  }
}