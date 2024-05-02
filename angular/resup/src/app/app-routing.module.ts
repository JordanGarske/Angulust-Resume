import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { SignupComponent } from './authentication/signup/signup.component';
import { LoginComponent } from './authentication/login/login.component';
import { HomeComponent } from './navbar-folder/home/home.component';
import { NavbarComponent } from './navbar/navbar.component';
import { RoadMapComponent } from './navbar-folder/road-map/road-map.component';
import { AdminComponent } from './admin/admin.component';
import { ReferenceFormComponent } from './navbar-folder/reference-form/reference-form.component';
import { ChatRoomComponent } from './chat-room/chat-room.component';
import { NightSkyComponent } from './navbar-folder/home/home_components/night-sky/night-sky.component';

const routes: Routes = [
  { path: '', redirectTo: '/login', pathMatch: 'full' },
  { path: 'sky', component: NightSkyComponent},
  { path: 'login', component: LoginComponent},
  { path: 'signup', component: SignupComponent },
  { path: 'admin', component: AdminComponent },
  //{ path: 'home', component: HomeComponent },
  {
    path: 'home',
    component: NavbarComponent,
    children:[
      { path: '', component: HomeComponent },
      { path: 'roadmap', component: RoadMapComponent },
      { path: 'reference', component: ReferenceFormComponent }, 
      { path: 'chat-room', component: ChatRoomComponent }, 
    ],
  }
];
@NgModule({
  declarations: [],
  imports: [ RouterModule.forRoot(routes) ],
  exports: [ RouterModule ]
})
export class AppRoutingModule { }
