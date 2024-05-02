import { Component } from '@angular/core';
import { Router } from '@angular/router';

@Component({
  selector: 'app-night-sky',
  templateUrl: './night-sky.component.html',
  styleUrls: ['./night-sky.component.scss']
})
export class NightSkyComponent {
  stars: Star[] = [];
  constructor( private router: Router){}
  ngOnInit(): void{
  
      this.createStars();                                                                         
  }

  createStars(): void {
    const count = 1500;
    const scene = document.querySelector('.scene') as HTMLElement;

    for (let i = 0; i < count; i++) {
      const star: Star = {
        left: Math.floor(Math.random() * window.innerWidth) + 'px',
        top: Math.floor(Math.random() * window.innerHeight) + 'px',
        width: (1 + Math.random() * 2) + 'px',
        height: (1 + Math.random() * 2) + 'px',
        animationDuration: (5 + Math.random() * 2) + 's',
        animationDelay: (Math.random() * 2) + 's'
      };

      this.stars.push(star);
    }
  }

    // placePerson(): void {
  //   let mountain = document.querySelector('.mountain') as HTMLElement;;
  //   let person = document.querySelector('.person') as HTMLElement;;
  //   if(person !== null && mountain !== null){
  //     let mountainHeight = mountain.offsetHeight; // Get the height of the mountain
  //     let personHeight = person.offsetHeight; // Get the height of the person
    
  //     person.style.position = "absolute";
  //     person.style.left = "40%"; // Position the person in the middle horizontally
  //     person.style.transform = "translateX(-50%)"; // Center the person horizontally
  //     person.style.top = mountainHeight - personHeight +25+ "px"; // Position the person above the mountain
  //   }

  // }
  
}
export interface Star  {
  left: string;
  top : string;
  width : string;
  height : string;
  animationDuration : string;
  animationDelay: string;
}