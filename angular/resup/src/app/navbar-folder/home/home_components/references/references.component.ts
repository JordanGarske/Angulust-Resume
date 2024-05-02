import { Component, OnInit } from '@angular/core';
import { NavigationEnd, Router } from '@angular/router';
import { filter } from 'rxjs/operators';
import { ClientReference } from 'src/app/models/reference';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-references',
  templateUrl: './references.component.html',
  styleUrls: ['./references.component.scss']
})
export class ReferencesComponent implements OnInit {
  references:ClientReference[] = [  {
    id: 1,
    email: 'john.doe@example.com',
    first_name: 'John',
    last_name: 'Doe',
    phone_number: '123-456-7890',
    profession: 'Software Engineer',
    company: 'Tech Co.',
    elucidation: 'John is a highly skilled software engineer with a strong problem-solving ability.',
    card_front: 'front',
  },
  {
    id: 2,
    email: 'jane.smith@example.com',
    first_name: 'Jane',
    last_name: 'Smith',
    profession: 'Data Scientist',
    company: 'Data Analytics Inc.',
    elucidation: 'Jane has a deep understanding of data science and is a valuable team member.',
    card_front: 'front',
  },
  {
    id: 3,
    email: 'bob.johnson@example.com',
    first_name: 'Bob',
    last_name: 'Johnson',
    phone_number: null,
    profession: 'Project Manager',
    company: 'Project Management Co.',
    elucidation: 'Bob is an experienced project manager who excels in leading teams to success.',
    card_front: 'front',
  },]
  leftReferences: ClientReference[] = [];
  rightReferences: ClientReference[] = [];
  isFlipped = 'front';
  constructor(private userService: UserService, private router: Router) {}

  ngOnInit(): void {
    this.reload();
    this.divideReference();
  }

  reload(): void {
    this.userService.getReference().subscribe(results => {
      this.references = results;
      this.divideReference();
    });
  }

  divideReference(): void {
    this.leftReferences = [];
    this.rightReferences = [];
    if (this.references.length % 2 === 0) {
      for (let i = 0; i < this.references.length; i += 2) {
        this.references[i].card_front = 'front'
        this.rightReferences.push(this.references[i]);
        this.references[i+1].card_front = 'front'
        this.leftReferences.push(this.references[i + 1]);
      }
    } else {
      const x: ClientReference = {
        id: 0,
        email: '',
        first_name: '',
        last_name: '',
        profession: '',
        company: '',
        elucidation: '',
        card_front: 'front',
      };
      this.references.push(x)
      for (let i = 0; i < this.references.length; i += 2) {
        this.references[i].card_front = 'front'
        this.rightReferences.push(this.references[i]);
        this.references[i+1].card_front = 'front'
        this.leftReferences.push(this.references[i + 1]);
      }
    }
  }
  toggleFlip(cli_ref: ClientReference){
    console.log(this.isFlipped);
     cli_ref.card_front = (cli_ref.card_front === 'front') ? 'back' : 'front';
  }
}
