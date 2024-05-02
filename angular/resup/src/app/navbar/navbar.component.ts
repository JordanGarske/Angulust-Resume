import { Component } from '@angular/core';
import { UserService } from '../services/user.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrls: ['./navbar.component.scss']
})
export class NavbarComponent {
  canWriteRef:boolean = false;
  constructor(private userService: UserService, private router: Router){}
  ngOnInit(): void{
      this.canWriteRef = this.userService.getCanWriteReference();
      this.placePerson();
      this.stars();                                                                         
  }
  placePerson(): void {
    let mountain = document.querySelector('.mountain') as HTMLElement;;
    let person = document.querySelector('.person') as HTMLElement;;
    if(person !== null && mountain !== null){
      let mountainHeight = mountain.offsetHeight; // Get the height of the mountain
      let personHeight = person.offsetHeight; // Get the height of the person
    
      person.style.position = "absolute";
      person.style.left = "40%"; // Position the person in the middle horizontally
      person.style.transform = "translateX(-50%)"; // Center the person horizontally
      person.style.top = mountainHeight - personHeight +25+ "px"; // Position the person above the mountain
    }

  }
  
  stars(): void {
    let count = 750;
    let scene = document.querySelector('.scene');
    let i = 0;
  
    while(i < count) {
  
      let star = document.createElement("i");
      let x = Math.floor(Math.random() * window.innerWidth);
      let y = Math.floor(Math.random() * window.innerHeight);
      let duration = Math.random() * 2;
      let size = Math.random() * 2; 
  
      star.style.left = x + 'px';
      star.style.top = y + 'px';
      star.style.width = 1 + size + 'px';
      star.style.height = 1 + size + 'px';
      star.style.animationDuration = 5 + duration +'s';  
      star.style.animationDelay = duration +'s';
      if(scene !== null){
        scene.appendChild(star);
      }
      i++;
  
    }
  }
  

}
