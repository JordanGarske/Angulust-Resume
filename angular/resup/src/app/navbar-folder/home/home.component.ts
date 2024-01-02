import { Component, OnInit } from '@angular/core';
import { NavigationEnd, Router } from '@angular/router';
import { filter } from 'rxjs/internal/operators/filter';
import { ClientReference } from 'src/app/models/reference';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit{
  references:ClientReference[] = [  {
    id: 1,
    email: 'john.doe@example.com',
    first_name: 'John',
    last_name: 'Doe',
    phone_number: '123-456-7890',
    profession: 'Software Engineer',
    company: 'Tech Co.',
    elucidation: 'John is a highly skilled software engineer with a strong problem-solving ability.',
  },
  {
    id: 2,
    email: 'jane.smith@example.com',
    first_name: 'Jane',
    last_name: 'Smith',
    profession: 'Data Scientist',
    company: 'Data Analytics Inc.',
    elucidation: 'Jane has a deep understanding of data science and is a valuable team member.',
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
  },]
  constructor(private userService: UserService, private router: Router){}
  ngOnInit(): void{
    this.userService.getReference().subscribe(results => this.references = results);
  }
}
